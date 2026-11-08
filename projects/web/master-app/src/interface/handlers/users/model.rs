use crate::model::entities::users::PostUserForm;
use axum::extract::{Form, State};
use axum::response::Redirect;
use core_library_db_queries::model::sessions::context::SessionContext;
use crate::infrastructure::states::AppState;

pub async fn insert_user_form(
    Form(_form): Form<PostUserForm>,
    State(_state): State<AppState>,
) {

}

pub async fn logout_handler(
    session: SessionContext,
    State(state): State<AppState>,
) -> Redirect {
    let _ = session.session.flush().await;

    Redirect::to("/")
}
