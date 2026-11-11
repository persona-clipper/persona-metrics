use askama::Template;
use axum::extract::State;
use axum::Form;
use axum::response::{IntoResponse, Redirect};
use serde::Serialize;
use tower_sessions::Session;
use tracing::instrument::WithSubscriber;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_types::store::users::password::Password;
use core_web_types::infrastructure::users::forms::{AuthorizationUserForm, SessionUserInfo};
use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/users/authorization.html")]
struct AuthorizationForm {
    pub error_message: Option<String>
}


pub async fn auth_user_handler(
    mut session: SessionContext,
    State(state): State<AppState>,
    Form(form): Form<AuthorizationUserForm>,
) -> Redirect {
    tracing::info!("{:#?}", form);

    let url = match state.api.read.url.join("api/v1/users/auth") {
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

    let _ = session.session.remove_value("message").await;
    match response {
        Ok(res) => {
            let response = res.json::<SessionUserInfo>().await;
            tracing::info!("Res: {:#?}", response);
            match response {
                Ok(user_info) => {
                    let _ = session.session.insert("user", &user_info).await;
                    session.user = Some(user_info);
                    tracing::info!("Session: {:#?}", session);

                    Redirect::to("/")
                },
                Err(err) => {
                    let _ = session.session.insert("message", "Не удалось авторизовать пользователя").await;
                    session.message = Some("Не удалось авторизовать пользователя".into());

                    Redirect::to("/login")
                }
            }
        },
        Err(err) => {
            let _ = session.session.insert("message", "Что-то пошло не так").await;
            session.message = Some("Что-то пошло не так".into());
            tracing::error!("Session: {:#?}", session);

            Redirect::to("/login")
        }
    }
}

pub async fn get_form(
    session: SessionContext,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let message = session.message.clone();
    let _ = session.session.remove_value("message").await;
    View {
        meta: PageMeta {
            title: "Авторизация".into(),
            flags: Flags {
                show_sidebar: false,
                show_header: false,
                ..Default::default()
            },
            session: Some(session.clone()),
            ..Default::default()
        },
        template: AuthorizationForm {
            error_message: message
        },
    }
}