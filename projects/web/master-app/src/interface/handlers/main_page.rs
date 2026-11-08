use crate::infrastructure::base_view::{PageMeta, View};
use crate::infrastructure::states::AppState;
use crate::model::entities::main_page::{MainPage, PhycisBallPage};
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use crate::infrastructure::sessions::context::SessionContext;

pub async fn get_main_page_handler(
    session: SessionContext,
    State(_state): State<AppState>,
) -> impl IntoResponse {



    View {
        meta: PageMeta {
            title: "Главная страница",
            description: Some("Главная страница проекта Persona Metrics"),
            session: Some(session.clone()),
            ..Default::default()
        },
        template: MainPage {
            text: "ОООООООООООООПППППППППППППППППППППППППППППАААААААААААААААААААААААААААА".to_string(),
            user_info: session.user,
        }
    }
}



pub async fn get_physic_ball(State(_state): State<AppState>) -> impl IntoResponse {
    View {
        meta: Default::default(),
        template: PhycisBallPage,
    }
}