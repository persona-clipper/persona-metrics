use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use core_library_db_queries::model::sessions::context::SessionContext;
use core_library_db_types::store::organizations::list::OrganizationListItem;
use crate::infrastructure::base_view::{PageMeta, View};
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
    
    
    
    View {
        meta: PageMeta {
            title: format!("Организации {}", session.clone().user.unwrap().username).into(),
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: OrganizationListView {
            items: vec![],
        },
    }
}