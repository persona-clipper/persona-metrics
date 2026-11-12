use serde::Deserialize;

#[derive(Deserialize)]
pub struct OrganizationInfo {
    pub unique_hash_id: String,
    pub display_name: String,
    pub members: Vec<OrganizationMemberInfo>
}

#[derive(Deserialize)]
pub struct OrganizationMemberInfo {
    pub user_id: i32,
    pub username: String,
    pub role: String,
}