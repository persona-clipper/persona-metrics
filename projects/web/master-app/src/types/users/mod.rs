use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostUserForm {
    pub name: String,
}