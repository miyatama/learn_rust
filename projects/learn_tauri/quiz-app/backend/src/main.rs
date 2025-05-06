// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_span_events(tracing_subscriber::fmt::format::FmtSpan::ACTIVE)
        .init();
    let span = tracing::debug_span!("debug_main", key = "backend");
    let _guard = span.enter();
    quiz_app_lib::run()
}
