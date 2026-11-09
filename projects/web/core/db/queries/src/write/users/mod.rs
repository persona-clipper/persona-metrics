use axum::extract::State;
use axum::{Form, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use sqlx::PgPool;
use core_library_db_types::store::users::password::Password;
use core_library_db_types::store::users::user::{CreateOrganizationForm, CreateUserRow};
use core_web_types::infrastructure::users::forms::{CreateUserResponse, RegistrationUserForm, SessionUserInfo};

pub async fn insert_user_query(
    form: CreateUserRow,
    db: PgPool,
) -> Result<SessionUserInfo, sqlx::Error> {
    sqlx::query_as!(
        SessionUserInfo,
        r#"
        INSERT INTO users(username, display_name, email, password_hash)
        VALUES($1, $2, $3, $4)
        RETURNING avatar_path, id, username
        "#,
        form.username,
        form.display_name.unwrap_or_else(|| "User".to_string()),
        form.email,
        form.password_hash
    ).fetch_one(&db).await
}

pub async fn insert_organization(
    form: &CreateOrganizationForm,
    user_id: i32,
    db: PgPool,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO organizations(unique_name, display_name, description, owner_user_id)
        VALUES($1, $2, $3, $4);
        "#,
        form.name,
        form.display_name,
        form.description,
        user_id,
    ).execute(&db).await?;

    Ok(())

}