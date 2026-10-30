use askama::Template;
use askama_web::__askama_web_impl::axum_core_0_5::IntoResponse;
use axum::extract::State;
use crate::modules::states::AppState;
use crate::types::main_page::MainPage;

pub async fn get_main_page_handler(
    State(state): State<AppState>,
) -> MainPage {
    MainPage {
        title: "БЛАБАЛАЛААЛБАБА".to_string(),
    }
}