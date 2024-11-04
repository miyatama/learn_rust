use bytes::Buf;
use futures::future::join_all;
use log::{debug, trace};
use reqwest;
use reqwest::header::CONTENT_TYPE;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io;
use std::io::Write;

#[derive(Serialize, Deserialize)]
struct AddParams {
    text: String,
}

pub async fn run_get_picture<'a>(
    url: &'a str,
    outfile: &'a str,
) -> Result<u16, Box<dyn std::error::Error>> {
    debug!("start run_get_picture");
    trace!("  url: {}", &url);
    trace!("  outfile: {}", &outfile);

    let res = reqwest::get(url).await?;
    let status_code = res.status().as_u16();
    let mut r = res.bytes().await?.reader();
    let mut w = File::create(outfile)?;
    let _ = io::copy(&mut r, &mut w);
    Ok(status_code)
}

pub async fn run_post(url: &str, outfile: &str) -> Result<u16, Box<dyn std::error::Error>> {
    debug!("start run_post");
    let param = AddParams {
        text: "message".to_string(),
    };
    let param_string = serde_json::to_string(&param).unwrap();
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header(CONTENT_TYPE, "application/json")
        .body(param_string)
        .send()
        .await?;
    let status_code = res.status().as_u16();
    let content = res.text().await?;
    let mut w = File::create(outfile)?;
    let _ = w.write_all(content.as_bytes());
    Ok(status_code)
}

pub async fn run_get_pictures() -> Result<(), Box<dyn std::error::Error>> {
    debug!("start run_get_pictures");
    /* use macro_rules format!
    macro_rules! image_filename_base {
        () => {
            "image-{}.jpg"
        };
    }
     */
    let download_infos = (10..20)
        .map(|index| {
            (
                format!("https://raw.githubusercontent.com/yavuzceliker/sample-images/refs/heads/main/images/image-{}.jpg", &index),
                format!("image-{}.jpg", &index),
            )
        })
        .collect::<Vec<(String, String)>>();

    let download_pictures = download_infos
        .iter()
        .map(|(url, image_name)| run_get_picture(&url, &image_name))
        .collect::<Vec<_>>();
    join_all(download_pictures).await;
    Ok(())
}
