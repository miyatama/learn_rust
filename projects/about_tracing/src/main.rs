use tracing::{event, Level, info, info_span};
fn main() {
    tracing_subscriber::fmt()
        .init();

    let span = info_span!("span_1", key="hello");
    let _guard = span.enter();

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
