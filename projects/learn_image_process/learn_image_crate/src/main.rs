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
    decoder_encoder();
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

use image::ImageDecoder;
fn decoder_encoder() {
    let file = match std::fs::File::open("cargo.png") {
        Ok(file) => file,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };
    let mut reader = std::io::BufReader::new(file);
    let mut decoder = match image::codecs::png::PngDecoder::new(&mut reader) {
        Ok(decoder) => decoder,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };
    let icc = decoder.icc_profile();
    log::info!("icc: {:?}", icc);
    let img = match image::DynamicImage::from_decoder(decoder) {
        Ok(img) => img,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };

    let mut writer = match std::fs::File::create("cargo3.png") {
        Ok(file) => file,
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
    };
    let encoder = image::codecs::png::PngEncoder::new(&mut writer);
    match img.write_with_encoder(encoder) {
        Err(e) => {
            log::error!("{:?}", e);
            return;
        }
        _ => {}
    }
}
