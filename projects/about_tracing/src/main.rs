use tracing::{event, Level, info};
fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    func01();
    func02();
}

#[tracing::instrument]
fn func01() {
    info!("func01");
    func_b_01();
}

// #[tracing::instrument(skip(self, hoge))]
fn func02() {
    info!("func02");
    func_b_01();
}

#[tracing::instrument]
fn func_b_01() {
    info!("func_b_01");
}
