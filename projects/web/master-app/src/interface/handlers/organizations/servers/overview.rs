use askama::Template;
use axum::extract::{Path, State};
use core_library_db_queries::model::sessions::context::SessionContext;
use crate::infrastructure::base_view::{PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/organizations/servers/overview.html")]
pub struct ServerOverview;

pub async fn get_overview_page<'a>(
    session: SessionContext,
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> View<'a, ServerOverview> {
    
    
   View {
       meta: PageMeta {
           title: Default::default(),
           description: None,
           flags: Default::default(),
           session: None,
       },
       template: ServerOverview,
   }
}