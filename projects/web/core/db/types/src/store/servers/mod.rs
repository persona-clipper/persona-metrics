use compact_str::CompactString;
use sqlx::FromRow;
use crate::store::users::User;

#[derive(FromRow)]
pub struct ServerObject {
    pub id: i64,
    pub display_name: CompactString,
    pub server_hash: String,
    pub owner: User,
    pub category: ServerCategory,
    pub is_active: bool,
}

#[derive(FromRow)]
pub struct ServerCategory {
    pub id: i64,
    pub name: String,
}