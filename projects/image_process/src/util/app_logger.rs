pub struct AppLogger {}

impl log::Log for AppLogger {
    fn enabled(&self, _meta: &log::Metadata) -> bool {
        // meta.target() == "json_log"
        true
    }

    fn log(&self, record: &log::Record) {
        if self.enabled(record.metadata()) {
            println!(r#"[{}]: {}"#, record.level(), record.args());
        }
    }

    fn flush(&self) {}
}
