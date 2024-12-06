use log::error;

fn main() {
    if let Err(e) = ui::get_args().and_then(ui::run) {
        error!("{}", e);
        std::process::exit(1);
    }
}
