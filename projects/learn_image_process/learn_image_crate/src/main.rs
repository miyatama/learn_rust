mod util;
use self::util::app_logger::AppLogger;
use image::ImageReader;
use log::{ LevelFilter};

static LOGGER: AppLogger = AppLogger {};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
    read_and_save();
}

fn read_and_save() {
    let reader = match ImageReader::open("cargo.png") {
        Ok(reader) => reader,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };
    let img = match reader.decode() {
        Ok(image) => image,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };
    let _ = img.save("cargo2.png");
}
