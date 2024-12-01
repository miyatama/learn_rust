use self::util::app_logger::AppLogger;
use log::LevelFilter;

pub mod util;

static LOGGER: AppLogger = AppLogger {};

#[tokio::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);

    multi_thread::run();
    multi_thread::run_async().await;
    multi_thread::run_mutex_channel();
    multi_thread::run_scoped_thread();
    multi_thread::run_error_trace();
    multi_thread::run_parallel_process();
    multi_thread::run_fork_join();
}
