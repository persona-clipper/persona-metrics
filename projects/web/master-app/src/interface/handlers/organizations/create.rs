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
use crate::infrastructure::json_web_tokens::generate_json_web_token;
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/organizations/create.html")]
pub struct CreateOrganizationFormView;

pub async fn create_organization_handler(
    session: SessionContext,
    State(state): State<AppState>,
    Form(payload): Form<CreateOrganizationForm>,
) -> Redirect {
    if let Some(user_info) = session.user {
        let claims = generate_json_web_token(&user_info, state.api.write.jwt_secret_token.clone());
        if let Ok(claims) = claims {
            let target_url = state.api.write.url.join("api/v1/users/orgs").expect("Failed to parse url");

            let response = state.http_client
                .post(target_url)
                .json(&payload)
                .bearer_auth(&claims)
                .send()
                .await;
            tracing::debug!("{:#?}", response);
        }
    }
    Redirect::to("/orgs")
}

pub async fn get_create_form(
    session: SessionContext,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Создание организации".into(),
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: CreateOrganizationFormView,
    }
}