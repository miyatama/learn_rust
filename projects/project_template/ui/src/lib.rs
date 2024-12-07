use clap::Parser;
use log::info;
use util::AppResult;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[arg(short, long)]
    number: u32,
}

pub fn run(config: &Config) -> AppResult<()> {
    info!("config: {:?}", config);
    Ok(())
}

pub fn get_args() -> AppResult<Config> {
    let args = Config::parse();
    Ok(args)
}
