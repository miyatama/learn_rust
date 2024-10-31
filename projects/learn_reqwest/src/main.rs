use std::io;
use std::env;
use std::fs::File;
use bytes::Buf;
use reqwest;
use futures::executor::block_on;

#[tokio::main]
async fn main() {
    let args : Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("argument not enough");
        return
    }
    let url = args[1].as_str();
    let outfile = args[2].as_str();
    match run_get(&url, &outfile).await {
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
