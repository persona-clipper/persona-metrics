use askama::Template;
use axum::extract::State;
use axum::Form;
use axum::response::{IntoResponse, Redirect};
use core_library_db_types::store::users::password::Password;
use core_web_types::infrastructure::users::forms::AuthorizationUserForm;
use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/users/authorization.html")]
struct AuthorizationForm;


pub async fn auth_user_handler(
    State(state): State<AppState>,
    Form(form): Form<AuthorizationUserForm>
) -> Redirect {
    tracing::info!("{:#?}", form);

    let url = match state.api.write_url.join("v1/users") {
        Ok(url) => url,
        Err(_) => {
            return Redirect::to("/signup")
        }
    };
    let response = state.http_client.post(url);

    todo!("Доделать авторизация");
    Redirect::to("/")
}

pub async fn get_form(
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