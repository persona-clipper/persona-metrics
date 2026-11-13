use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct OrganizationInfo {
    pub unique_hash_id: String,
    pub display_name: String,
    pub description: Option<String>,
    pub created_at: chrono::DateTime<Utc>,
    pub owner_username: String,
    pub members: Vec<OrganizationMemberInfo>,
}

#[derive(Deserialize, Serialize)]
pub struct OrganizationMemberInfo {
    pub user_id: i32,
    pub display_name: String,
    pub username: String,
    pub avatar_path: Option<String>,
    pub is_active: bool,
    pub role_id: i32,
    pub role_name: String,
}