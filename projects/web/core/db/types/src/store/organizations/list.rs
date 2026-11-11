use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct OrganizationListItem {
    pub display_name: String,
    pub description: Option<String>,
    pub role: String,
}
