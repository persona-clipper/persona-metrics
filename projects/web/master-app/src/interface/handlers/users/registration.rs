use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::states::AppState;
use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use tower_sessions::Session;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_types::store::users::password::{Password, PasswordError};
use core_library_db_types::store::users::user::{User, UserError};
use core_web_types::infrastructure::users::forms::{CreateUserResponse, RegistrationUserForm, SessionUserInfo};

#[derive(Template)]
#[template(path = "pages/users/registration.html")]
struct RegistrationForm {
    error_message: Option<String>,
}

pub async fn create_user_handler(
    mut session: SessionContext,
    State(state): State<AppState>,
    Form(form): Form<RegistrationUserForm>,
) -> Redirect {
    tracing::debug!("{:#?}", form);
    let _ = session.session.remove_value("message").await;

    if let Err(err) = Password::new(&form.password).check(&form.password_confirm) {
        match err {
            PasswordError::PasswordMismatch => {
                let _ = session.session.insert("message", "Пароли не совпадают").await;
            },
            PasswordError::PasswordNotEnoughLength => {
                let _ = session.session.insert("message", "Пароль недостаточной длиины").await;
            }
            _ => {}
        }
        return Redirect::to("/signup");
    }
    // if let Err(err) = Password::new(&form.password).check(&form.password_confirm).is_err() {
    //     return Redirect::to("/signup");
    // }
    if let Err(err) = User::new(&form.username) {
        match err {
            UserError::UserInvalidCharactersInUsername { .. } => {
                let _ = session.session.insert("message", "Недопустимые символы в имени пользователя").await;
            },
            UserError::UserUsernameNotEnoughLength { .. } => {
                let _ = session.session.insert("message", "Недостаточна длина имени. Минимум 8 символов").await;
            }
            _ => {}
        }
        tracing::warn!("Username incorrect: {:#?}", err);
        return Redirect::to("/signup")
    }

    let url = match state.api.write.url.join("api/v1/users") {
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
        Ok(response) => {
            let user_info = response.json::<SessionUserInfo>().await;
            if let Ok(info) = user_info {
                let _ = session.session.insert("user", &info).await;
                tracing::info!("User registration success: id {:#?}", info.id);
                session.user = Some(info);
            }
        },
        Err(err) => {
            tracing::error!("Error creating user: {:?}", err);
            let _ = session.session.insert("message", "Не удалось зарегистрироваться").await;
            session.message = Some("Не удалось зарегистрироваться".into());
            return Redirect::to("/signup")
        }
    }

    Redirect::to("/")
}

pub async fn get_form(
    session: SessionContext,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    let message = session.message.clone();
    let _ = session.session.remove_value("message").await;
    View {
        meta: PageMeta {
            title: "Регистрация".into(),
            description: Some("Страница регистрации"),
            flags: Flags {
                show_header: false,
                show_sidebar: false,
                show_footer: true,
            },
            session: Some(session.clone()),
            ..Default::default()
        },
        template: RegistrationForm {
            error_message: message
        },
    }
}