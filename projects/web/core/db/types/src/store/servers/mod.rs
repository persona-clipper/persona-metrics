use compact_str::CompactString;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use crate::store::users::User;

#[derive(FromRow, Serialize, Deserialize)]
pub struct ServerObject {
    pub id: i64,
    pub display_name: CompactString,
    pub server_hash: String,
    #[serde(flatten)]
    pub owner: User,
    pub category: ServerCategory,
    pub is_active: bool,
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct ServerCategory {
    pub id: i64,
    pub name: CompactString,
}