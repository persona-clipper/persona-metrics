use master_app::modules::states::AppState;

fn main() {
    tracing_subscriber::fmt::init();

    let state = AppState::new();
}
