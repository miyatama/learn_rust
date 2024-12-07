use log::error;
use log::LevelFilter;
use util::AppLogger;

static LOGGER: AppLogger = AppLogger {};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    match ui::get_args() {
        Ok(config) => {
            // TODO ログレベル置き換え
            log::set_max_level(LevelFilter::Trace);

            if let Err(e) = ui::run(&config) {
                error!("{}", e);
                std::process::exit(1);
            }
        }
        Err(e) => {
            error!("{}", e);
            std::process::exit(1);
        }
    }
}
