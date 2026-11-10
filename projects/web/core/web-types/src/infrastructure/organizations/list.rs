use serde::Deserialize;

#[derive(Deserialize)]
pub struct OrganizationListItem {
    pub display_name: String,
    pub description: String,
    pub role: String,
}
