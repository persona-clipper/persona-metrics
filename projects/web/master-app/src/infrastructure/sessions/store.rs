use std::fmt::Write;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::{Executor, PgPool, SqlSafeStr};
use tower_sessions::session::{Id, Record};
use tower_sessions::{ExpiredDeletion, SessionStore};

#[derive(Debug, Clone)]
pub struct PostgresStore {
    pub pool: PgPool,
    pub schema_name: String,
    pub table_name: String,
}

impl PostgresStore {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            schema_name: "tower-session".into(),
            table_name: "session".into(),
        }
    }

    pub async fn migrate(&self) -> sqlx::Result<()> {
        let mut tx = self.pool.begin().await?;

        let create_schema_query = format!(
            r#"
            CREATE SCHEMA IF NOT EXISTS "{}"
            "#,
            self.schema_name
        );

        sqlx::query(sqlx::AssertSqlSafe(create_schema_query)).execute(&mut *tx).await?;

        let create_table_query = format!(
            r#"
            CREATE TABLE IF NOT EXISTS "{}"."{}"
            (
                id TEXT PRIMARY KEY NOT NULL,
                data BYTEA NOT NULL,
                expiry_date TIMESTAMPTZ NOT NULL
            );
            "#,
            self.schema_name, self.table_name
        );

        sqlx::query(sqlx::AssertSqlSafe(create_table_query)).execute(&mut *tx).await?;

        tx.commit().await?;

        Ok(())
    }
}

#[async_trait]
impl SessionStore for PostgresStore {
    async fn save(&self, session_record: &Record) -> tower_sessions::session_store::Result<()> {
        let mut buffer: Vec<u8> = Vec::new();
        let mut serializer = rmp_serde::Serializer::new(&mut buffer)
            .with_struct_map();

        session_record
            .serialize(&mut serializer)
            .map_err(|e| tower_sessions::session_store::Error::Encode(e.to_string()))?;


        // let data = rmp_serde::to_vec(&session_record.data)
        //     .map_err(|e| tower_sessions::session_store::Error::Encode(e.to_string()))?;

        let expiry: chrono::DateTime<chrono::Utc> = chrono::DateTime::from_timestamp(
            session_record.expiry_date.unix_timestamp(),
            session_record.expiry_date.nanosecond(),
        ).unwrap_or_default();

        let query = format!(
            r#"
            INSERT INTO "{}"."{}" (id, data, expiry_date)
            VALUES ($1, $2, $3)
            ON CONFLICT (id) DO UPDATE
            SET data = EXCLUDED.data, expiry_date = EXCLUDED.expiry_date
            "#,
            self.schema_name, self.table_name
        );

        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(session_record.id.to_string())
            .bind(buffer)
            .bind(expiry)
            .execute(&self.pool)
            .await
            .map_err(|e| tower_sessions::session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }

    async fn load(&self, session_id: &Id) -> tower_sessions::session_store::Result<Option<Record>> {
        let query = format!(
            r#"
            SELECT data FROM "{}"."{}"
            WHERE id = $1 AND expiry_date > NOW();
            "#,
            self.schema_name, self.table_name
        );

        let session_data: Option<(Vec<u8>,)> = sqlx::query_as(sqlx::AssertSqlSafe(query))
            .bind(session_id.to_string())
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| tower_sessions::session_store::Error::Backend(e.to_string()))?;

        match session_data {
            Some((data,)) => {
                let data: Record = rmp_serde::from_slice(&data)
                    .map_err(|e| tower_sessions::session_store::Error::Decode(e.to_string()))?;

                Ok(Some(data))
            },
            None => Ok(None)
        }
    }

    async fn delete(&self, session_id: &Id) -> tower_sessions::session_store::Result<()> {
        let query = format!(
            r#"
            DELETE FROM "{}"."{}" WHERE id = $1
            "#,
            self.schema_name, self.table_name
        );

        sqlx::query(sqlx::AssertSqlSafe(query))
            .bind(session_id.to_string())
            .execute(&self.pool)
            .await
            .map_err(|e| tower_sessions::session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl ExpiredDeletion for PostgresStore {
    async fn delete_expired(&self) -> tower_sessions::session_store::Result<()> {
        let query = format!(
            r#"
            DELETE FROM "{}"."{}" WHERE expiry_date <= (NOW() AT TIME ZONE 'UTC');
            "#,
            self.schema_name, self.table_name
        );

        sqlx::query(sqlx::AssertSqlSafe(query))
            .execute(&self.pool)
            .await
            .map_err(|e| tower_sessions::session_store::Error::Backend(e.to_string()))?;

        Ok(())
    }
}
