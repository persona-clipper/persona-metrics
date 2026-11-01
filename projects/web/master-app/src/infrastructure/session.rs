use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct UserInfo {
    pub id: i32,
    pub avatar_path: String,

}
#[derive(Serialize, Deserialize)]
pub struct SessionContext {
    pub message: String,
    pub user_info: UserInfo,
}