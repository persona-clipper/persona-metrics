use askama::Template;
use axum::extract::State;
use core_library_db_queries::model::sessions::context::SessionContext;
use crate::infrastructure::base_view::{PageMeta, View};
use crate::infrastructure::states::AppState;

#[derive(Template)]
#[template(path = "pages/organizations/current/info.html")]
pub struct CurrentOrganization;

pub async fn get_page<'a>(
    session: SessionContext,
    State(state): State<AppState>
) -> View<'a, CurrentOrganization> {
    View {
        meta: PageMeta {
            title: "Организация".into(),
            description: None,
            flags: Default::default(),
            session: Some(session.clone()),
        },
        template: CurrentOrganization,
    }
}