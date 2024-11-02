use bytes::Buf;
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

#[tokio::main]
async fn main() {
    let get_url = "https://raw.githubusercontent.com/yavuzceliker/sample-images/refs/heads/main/images/image-1.jpg";
    let get_result_file = "image-1.jpg";
    match run_get_picture(&get_url, &get_result_file).await {
        Ok(status_code) => println!("download success! status is {}", status_code),
        Err(err) => eprintln!("download failed.{:?}", err),
    }

    let post_url = "http://localhost:8080/update_memo";
    let post_result_file = "post_result.json";
    match run_post(&post_url, &post_result_file).await {
        Ok(status_code) => println!("post method! status is {}", status_code),
        Err(err) => eprintln!("post method failed.{:?}", err),
    }
}

async fn run_get_picture(url: &str, outfile: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let status_code = res.status().as_u16();
    let mut r = res.bytes().await?.reader();
    let mut w = File::create(outfile)?;
    io::copy(&mut r, &mut w);
    Ok(status_code)
}

async fn run_post(url: &str, outfile: &str) -> Result<u16, Box<dyn std::error::Error>> {
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
    let mut content = res.text().await?;
    let mut w = File::create(outfile)?;
    w.write_all(content.as_bytes());
    Ok(status_code)
}
