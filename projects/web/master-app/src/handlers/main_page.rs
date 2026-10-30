use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use crate::modules::states::AppState;
use crate::types::common::{PageMeta, View};
use crate::types::main_page::{MainPage, RegistationForm};

pub async fn get_main_page_handler(
    State(state): State<AppState>,
) -> impl IntoResponse {
    View {
        meta: PageMeta {
            title: "Главная страница".to_string(),
        },
        template: MainPage {
            text: "ОООООООООООООПППППППППППППППППППППППППППППАААААААААААААААААААААААААААА".to_string(),
        }
    }
}

pub async fn get_registration_form_handler(
    State(state): State<AppState>,
) -> RegistationForm {
    RegistationForm {}
}