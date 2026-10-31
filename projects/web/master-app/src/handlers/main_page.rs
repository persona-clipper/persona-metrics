use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use crate::modules::states::AppState;
use crate::types::common::{PageMeta, View};
use crate::types::main_page::{MainPage, PhycisBallPage, RegistrationForm};

pub async fn get_main_page_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Главная страница",
            description: Some("Главная страница проекта Persona Metrics"),
            ..Default::default()
        },
        template: MainPage {
            text: "ОООООООООООООПППППППППППППППППППППППППППППАААААААААААААААААААААААААААА".to_string(),
        }
    }
}

pub async fn get_registration_form_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Регистрация",
            ..Default::default()
        },
        template: RegistrationForm,
    }
}

pub async fn get_physic_ball(State(state): State<AppState>) -> impl IntoResponse {
    View {
        meta: Default::default(),
        template: PhycisBallPage,
    }
}