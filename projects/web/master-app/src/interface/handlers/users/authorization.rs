use askama::Template;
use axum::extract::State;
use axum::Form;
use axum::response::{IntoResponse, Redirect};
use serde::Serialize;
use tower_sessions::Session;
use core_library_db_types::store::users::password::Password;
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, SessionUserInfo};
use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::session::SessionContext;
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/users/authorization.html")]
struct AuthorizationForm;


pub async fn auth_user_handler(
    mut session: SessionContext,
    State(state): State<AppState>,
    Form(form): Form<AuthorizationUserForm>,
) -> Redirect {
    tracing::info!("{:#?}", form);

    let url = match state.api.read_url.join("v1/users/auth") {
        Ok(url) => url,
        Err(_) => {
            return Redirect::to("/signup")
        }
    };
    let response = state.http_client
        .post(url)
        .json(&form)
        .send()
        .await;

    match response {
        Ok(res) => {
            let response = res.json::<SessionUserInfo>().await;
            tracing::info!("Res: {:#?}", response);
            if let Ok(auth_info) = response {
                let _ = session.session.insert("user", &auth_info).await;
                session.user = Some(auth_info);
            }
            tracing::info!("Session: {:#?}", session);
        },
        Err(err) => {
            let _ = session.session.insert("message", "Что-то пошло не так".to_string()).await;
            tracing::info!("Session: {:#?}", session);
            return Redirect::to("/login");
        }
    }

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