use std::time::{SystemTime, UNIX_EPOCH};
use askama::filters::json;
use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use axum::Form;
use axum::response::Redirect;
use jsonwebtoken::{EncodingKey, Header};
use core_library_db_queries::model::json_token::context::Claims;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_types::store::users::user::CreateOrganizationForm;
use crate::infrastructure::base_view::{PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/organizations/create_organization.html")]
pub struct CreateOrganizationFormView;

pub async fn create_organization_handler(
    session: SessionContext,
    State(state): State<AppState>,
    Form(payload): Form<CreateOrganizationForm>,
) -> Redirect {
    if let Some(user_info) = session.user {
        let expiry = SystemTime::now().duration_since(UNIX_EPOCH).expect("Failed to get system time").as_secs();
        let claims = Claims {
            sub: user_info.id,
            username: user_info.username,
            // К текущему времени прибавляем 800 секунд, это будет время когда токен истечет
            exp: expiry + 800,
        };
        let secret = EncodingKey::from_secret(&state.api.write.jwt_secret_token.into_bytes());
        let claims = jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &secret
        ).expect("");

        let target_url = state.api.write.url.join("v1/users/orgs").expect("Failed to parse url");

        let response = state.http_client
            .post(target_url)
            .json(&payload)
            .bearer_auth(&claims)
            .send()
            .await;

        match response {
            Ok(val) => {

            }
            Err(err) => {

            }
        }
    }

    Redirect::to("/")
}

pub async fn get_create_form(
    session: SessionContext,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Создание организации",
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: CreateOrganizationFormView,
    }
}