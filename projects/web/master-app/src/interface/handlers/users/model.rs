use crate::model::entities::users::PostUserForm;
use axum::extract::{Form, State};
use crate::infrastructure::states::AppState;

pub async fn insert_user_form(
    Form(_form): Form<PostUserForm>,
    State(_state): State<AppState>,
) {

}
