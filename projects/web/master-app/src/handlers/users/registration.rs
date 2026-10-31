use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use axum::{Form, Json};
use axum::response::Redirect;
use core_web_types::modules::users::forms::{CreateUserResponse, RegistrationUserForm};
use crate::modules::states::AppState;

pub async fn registration_user_handler(
    State(state): State<AppState>,
    Form(form): Form<RegistrationUserForm>,
) -> impl IntoResponse {
   let response = state.http_client
       .post(format!("{}/v1/users", state.api.write_url))
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