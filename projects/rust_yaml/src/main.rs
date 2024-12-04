use self::util::app_logger::AppLogger;
use log::LevelFilter;

pub mod util;

static LOGGER: AppLogger = AppLogger {};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    rust_yaml::run_example();
    rust_yaml::run_yaml_struct();
}
