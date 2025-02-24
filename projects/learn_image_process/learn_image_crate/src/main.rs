mod util;
use self::util::app_logger::AppLogger;
use image::ImageReader;
use log::LevelFilter;

static LOGGER: AppLogger = AppLogger {};

fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
    read_and_save();
    create_bytes();
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

fn create_bytes() {
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
    let mut bytes: Vec<u8> = Vec::new();
    match img.write_to(
        &mut std::io::Cursor::new(&mut bytes),
        image::ImageFormat::Png,
    ) {
        Ok(_) => {
            log::info!("bytes: {:?}", bytes);
        }
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    }
}
