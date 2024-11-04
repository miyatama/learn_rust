use self::logics::api;
use self::util::app_logger::AppLogger;
use log::{error, info, LevelFilter};

pub mod logics;
pub mod util;

static LOGGER: AppLogger = AppLogger {};

#[tokio::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Trace);
    log::trace!("trace");
    log::debug!("debug");
    log::info!("info");
    log::warn!("warn");
    log::error!("error");

    let get_url = "https://raw.githubusercontent.com/yavuzceliker/sample-images/refs/heads/main/images/image-1.jpg";
    let get_result_file = "image-1.jpg";
    match api::run_get_picture(&get_url, &get_result_file).await {
        Ok(status_code) => info!("download success! status is {}", status_code),
        Err(err) => error!("download failed.{:?}", err),
    }

    match api::run_get_pictures().await {
        Ok(_) => info!("download pictures success!"),
        Err(err) => error!("download pictures failed.{:?}", err),
    }

    let post_url = "http://localhost:8080/update_memo";
    let post_result_file = "post_result.json";
    match api::run_post(&post_url, &post_result_file).await {
        Ok(status_code) => info!("post method! status is {}", status_code),
        Err(err) => error!("post method failed.{:?}", err),
    }
}
