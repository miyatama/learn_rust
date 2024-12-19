use clap::{Parser, Subcommand};
use log::info;
use std::cmp::min;
use usecase::{
    AddTodoUseCase, DeleteTodoUseCase, GetTodoListUseCase, UpdateTodoUseCase, UseCases,
    UseCasesImpls,
};
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
        #[clap(short = 'n', long = "number", required = true, ignore_case = true)]
        number: u32,
    },
    Add {
        #[clap(short = 't', long = "text", required = true, ignore_case = true)]
        text: String,
    },
    Update {
        #[clap(long = "id", required = true, ignore_case = true)]
        id: u32,

        #[clap(short = 't', long = "text", required = true, ignore_case = true)]
        text: String,
    },
    Delete {
        #[clap(long = "id", required = true, ignore_case = true)]
        id: u32,
    },
}

pub async fn run(config: &Config) -> AppResult<()> {
    info!("config: {:?}", config);

    let usecases = UseCasesImpls::new().await;
    match &config.subcommand {
        SubCommands::List { number } => {
            let usecase = usecases.get_todo_list();
            match usecase.run() {
                Ok(todos) => {
                    let number = *number as usize;
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
        SubCommands::Add { text } => {
            let usecase = usecases.add_todo();
            match usecase.run(text.clone()) {
                Ok(todo) => {
                    info!("add succeed: {} - {}", &todo.id, &todo.text);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        SubCommands::Update { id, text } => {
            let usecase = usecases.update_todo();
            match usecase.run(*id, text.clone()) {
                Ok(todo) => {
                    info!("update succeed: {} - {}", &todo.id, &todo.text);
                    return Ok(());
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
        SubCommands::Delete { id } => {
            let usecase = usecases.delete_todo();
            return usecase.run(*id);
        }
    }
}

pub fn get_args() -> AppResult<Config> {
    let args = Config::parse();
    Ok(args)
}
