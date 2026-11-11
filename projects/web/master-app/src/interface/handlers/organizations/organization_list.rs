use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
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

pub async fn get_page(
    session: SessionContext,
    State(state): State<AppState>,
) -> impl IntoResponse {

    let target_url = state.api.read.url.join("api/v1/me/orgs").unwrap();
    let jwt = generate_json_web_token(&session.clone().user.unwrap(), state.api.read.jwt_secret_token).unwrap();
    let response = state.http_client
        .get(target_url)
        .bearer_auth(jwt)
        .send()
        .await
        .expect("Failed to get data from API");

    tracing::info!("{:#?}", response);
    let organization_list = response.json::<Vec<OrganizationListItem>>().await.unwrap_or_default();

    tracing::info!("{:#?}", organization_list);
    View {
        meta: PageMeta {
            title: format!("Организации {}", session.clone().user.unwrap().username).into(),
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: OrganizationListView {
            items: organization_list,
        },
    }
}