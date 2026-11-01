use askama::Template;
use axum::extract::State;
use axum::Form;
use axum::response::IntoResponse;
use core_web_types::infrastructure::users::forms::AuthorizationUserForm;
use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/users/authorization.html")]
struct AuthorizationForm;

pub async fn get_authorization_user_form(
    State(state): State<AppState>,
) -> impl IntoResponse  {
    View {
        meta: PageMeta {
            title: "Авторизация",
            flags: Flags {
                show_sidebar: false,
                show_header: false,
                ..Default::default()
            },
            ..Default::default()
        },
        template: AuthorizationForm,
    }
}