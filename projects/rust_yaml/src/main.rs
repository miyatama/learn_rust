use self::util::app_logger::AppLogger;
use log::LevelFilter;
use yaml_rust2::{YamlEmitter, YamlLoader};

pub mod util;

static LOGGER: AppLogger = AppLogger {};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    rust_yaml::run_example();
}
