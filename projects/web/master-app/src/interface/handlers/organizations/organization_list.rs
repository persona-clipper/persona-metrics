use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use axum::response::Redirect;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use crate::infrastructure::base_view::{PageMeta, View};
use crate::infrastructure::json_web_tokens::generate_json_web_token;
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/organizations/list.html")]
pub struct OrganizationListView {
    pub items: Vec<OrganizationListItem>,
}

pub async fn get_page<'a>(
    session: SessionContext,
    State(state): State<AppState>,
) -> Result<View<'a, OrganizationListView>, Redirect> {

    // if let Some(user_info) = session.user {
    //     if let Ok(jwt) = generate_json_web_token(&user_info, state.api.read.jwt_secret_token) {
    //         if let Ok(url) = state.api.read.url.join("api/v1/me/orgs") {

    //         }
    //     }
    // }

    let user_info = session.user
        .as_ref()
        .ok_or_else(|| {
            tracing::error!("Failed to get user info from sessions");
            Redirect::to("/")
        })?;
    let target_url = state.api.read.url.join("api/v1/me/orgs")
        .map_err(|e| {
            tracing::error!("Failed to parse target URL. Error: {}", e);
            Redirect::to("/")
        })?;
    let jwt = generate_json_web_token(user_info, state.api.read.jwt_secret_token)
        .map_err(|e| {
            tracing::error!("Failed to generate JWT token. Error: {}", e);
            Redirect::to("/")
        })?;

    let response = state.http_client
        .get(target_url)
        .bearer_auth(jwt)
        .send()
        .await
        .expect("Failed to get data from API");

    tracing::info!("{:#?}", response);
    let organization_list = response.json::<Vec<OrganizationListItem>>().await.unwrap_or_default();

    tracing::info!("{:#?}", organization_list);
    let view = View {
        meta: PageMeta {
            title: format!("Организации {}", user_info.username).into(),
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: OrganizationListView {
            items: organization_list,
        },
    };

    Ok(view)
}