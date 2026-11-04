use crate::infrastructure::base_view::{Flags, PageMeta, View};
use crate::infrastructure::states::AppState;
use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::Form;
use axum::extract::State;
use axum::response::Redirect;
use tower_sessions::Session;
use core_library_db_types::store::users::password::Password;
use core_library_db_types::store::users::user::User;
use core_web_types::infrastructure::users::forms::{CreateUserResponse, RegistrationUserForm};
use crate::infrastructure::session::SessionContext;

#[derive(Template)]
#[template(path = "pages/users/registration.html")]
struct RegistrationForm;

pub async fn create_user_handler(
    mut session: SessionContext,
    State(state): State<AppState>,
    Form(form): Form<RegistrationUserForm>,
) -> Redirect {
    tracing::info!("{:#?}", form);

    if Password::new(&form.password).check(&form.password_confirm).is_err() {
        return Redirect::to("/signup");
    }
    if let Err(err) = User::new(&form.username) {
        tracing::warn!("Username incorrect: {:#?}", err);
        return Redirect::to("/signup")
    }

    let url = match state.api.write_url.join("v1/users") {
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
            let id = response.json::<CreateUserResponse>().await;
            tracing::info!("User registration success: id {:#?}", id);
        },
        Err(err) => {
            tracing::error!("Error creating user: {:?}", err);
        }
    }

    Redirect::to("/")
}

pub async fn get_form(
    session: SessionContext,
    State(_state): State<AppState>,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Регистрация",
            description: Some("Страница регистрации"),
            flags: Flags {
                show_header: false,
                show_sidebar: false,
                show_footer: true,
            },
            session: Some(session),
            ..Default::default()
        },
        template: RegistrationForm,
    }
}