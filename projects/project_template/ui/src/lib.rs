use clap::{Parser, Subcommand};
use log::info;
use usecase::{GetTodoListUseCase, UseCases, UseCasesImpls};
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
        SubCommands::List { number } => match usecases.get_todo_list_usecase().run() {
            Ok(todos) => {
                for i in 0..number {
                    let todo = &todos[i as usize];
                    info!("{} - {}", todo.id, todo.text);
                }
                return Ok(());
            }
            Err(e) => {
                return Err(e);
            }
        },
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
