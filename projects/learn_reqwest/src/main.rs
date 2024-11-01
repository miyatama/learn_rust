use std::io;
use std::env;
use std::fs::File;
use bytes::Buf;
use reqwest;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let get_url = "https://raw.githubusercontent.com/yavuzceliker/sample-images/refs/heads/main/images/image-1.jpg" 
    let get_result_file = "image-1.jpg"
    match run_get(&get_url, &get_result_file).await {
        Ok(status_code) => println!("download success! status is {}", status_code),
        Err(err) => eprintln!("download failed.{:?}", err),
    }

    match run_post(&url, &outfile).await {
        Ok(status_code) => println!("download success! status is {}", status_code),
        Err(err) => eprintln!("download failed.{:?}", err),
    }
}

async fn run_get(url: &str, outfile: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let status_code = res.status().as_u16();
    let mut r = res.bytes().await?.reader();
    let mut w = File::create(outfile)?;
    io::copy(&mut r, &mut w);
    Ok(status_code)
}

async fn run_post(url: &str, outfile: &str) -> Result<u16, Box<dyn std::error::Error>> {
    let res = reqwest::get(url).await?;
    let status_code = res.status().as_u16();
    let mut r = res.bytes().await?.reader();
    let mut w = File::create(outfile)?;
    io::copy(&mut r, &mut w);
    Ok(status_code)
}