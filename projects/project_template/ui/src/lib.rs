use clap::{Parser, Subcommand};
use log::info;
use std::cmp::min;
use std::sync::Arc;
use usecase::{GetTodoListUseCase, GetTodoListUseCaseImpl, UseCases, UseCasesImpls};
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

pub async fn run(config: &Config) -> AppResult<()> {
    info!("config: {:?}", config);

    let usecases = UseCasesImpls::new().await;
    match config.subcommand {
        SubCommands::List { number } => {
            let usecase = usecases.get_todo_list();
            match usecase.run().await {
                Ok(todos) => {
                    let number = number as usize;
                    let max_index = min(number, todos.len());
                    for i in 0..max_index {
                        let todo = &todos[i];
                        info!("{} - {}", todo.id, todo.text);
                    }
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
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
