use clap::{Parser, Subcommand};
use log::info;
use usecase::UseCases;
use usecase::UseCasesImpls;
use util::AppResult;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Config {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, Subcommand)]
enum SubCommands {
    List {
        #[clap(short = 'l', long = "number", required = true, ignore_case = true)]
        number: u32,
    },
    Add,
    Update,
}

pub fn run(config: &Config) -> AppResult<()> {
    info!("config: {:?}", config);

    let usecases = UseCasesImpls::default();
    match config.subcommand {
        SubCommands::List { number } => {
            info!("call list.number: {}", number);
        }
        SubCommands::Add => {
            info!("call add");
        }
        SubCommands::Update => {
            info!("call update");
        }
    }
    Ok(())
}

pub fn get_args() -> AppResult<Config> {
    let args = Config::parse();
    Ok(args)
}
