use axum::extract::{State, Form};
use crate::modules::states::AppState;
use crate::types::users::PostUserForm;

pub async fn insert_user_form(
    Form(form): Form<PostUserForm>,
    State(state): State<AppState>,
) {

}
