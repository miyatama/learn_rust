#[derive(Debug, clap::Parser)]
#[command(version, about, long_about = None)]
struct Config {
    #[clap(subcommand)]
    subcommand: SubCommands,
}

#[derive(Debug, clap::Subcommand)]
enum SubCommands {
    Query,
    Add {
        #[clap(short = 't', long = "text", required = true, ignore_case = true)]
        text: String,
    },
    Subscribe,
}

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.json",
    query_path = "./src/query.graphql",
    normalization = "rust"
)]
pub struct TodoRepositories;

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.json",
    query_path = "./src/mutation.graphql",
    normalization = "rust"
)]
pub struct TodoUpdateRepositories;

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path = "./src/schema.json",
    query_path = "./src/subscription.graphql"
)]
struct TodoChanged;

mod util;
use self::util::app_logger::AppLogger;
static LOGGER: AppLogger = AppLogger {};

use clap::Parser;
use futures::StreamExt;
use graphql_ws_client::graphql::StreamingOperation;

#[async_std::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("start graphql client");

    let config = Config::parse();
    match config.subcommand {
        SubCommands::Query => {
            query();
        }
        SubCommands::Add { text } => {
            add_todo(text);
        }
        SubCommands::Subscribe => {
            subscribe().await;
        }
    }
}

fn query() {
    log::info!("start query");
    let client = reqwest::blocking::Client::new();
    let variables = todo_repositories::Variables {};

    let response_body = graphql_client::reqwest::post_graphql_blocking::<TodoRepositories, _>(
        &client,
        "http://localhost:3000/",
        variables,
    );
    match response_body {
        Ok(response) => match response.data {
            Some(data) => {
                let data = data as <TodoRepositories as graphql_client::GraphQLQuery>::ResponseData;
                data.todos.iter().for_each(|todo| {
                    log::info!("id: {}, text: {}", todo.id, todo.text);
                });
            }
            None => {
                log::info!("data is none");
            }
        },
        Err(err) => {
            log::error!("{:?}", err);
        }
    }
}

fn add_todo(text: String) {
    let client = reqwest::blocking::Client::new();
    let variables = todo_update_repositories::Variables { text: Some(text) };

    let response_body = graphql_client::reqwest::post_graphql_blocking::<TodoUpdateRepositories, _>(
        &client,
        "http://localhost:3000/",
        variables,
    );
    match response_body {
        Ok(response) => match response.data {
            Some(data) => {
                let data =
                    data as <TodoUpdateRepositories as graphql_client::GraphQLQuery>::ResponseData;
                log::info!("add todo id: {}", data.add_todo);
            }
            None => {
                log::info!("data is none");
            }
        },
        Err(err) => {
            log::error!("{:?}", err);
        }
    }
}

async fn subscribe() {
    /*
    let mut request = "ws://localhost:3000/ws".into_client_request().unwrap();
    use async_tungstenite::{
        async_std::connect_async,
        tungstenite::{client::IntoClientRequest, http::HeaderValue},
    };
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
    );
    let (connection, response) = connect_async(request).await.unwrap();
    log::info!("connected");
    log::info!("connect_async response: {:?}", response);

    let (client, _actor) = graphql_ws_client::Client::build(connection).await.unwrap();
    log::info!("client: {:?}", client);
    // log::info!("client build result: {:?}", actor);
    let stream = client
        .subscribe(StreamingOperation::<TodoChanged>::new(
            todo_changed::Variables,
        ))
        .await;
    log::info!("creating subscribe");
    let mut stream = match stream {
        Ok(stream) => {
            log::info!("created subscribe");
            stream
        }
        Err(err) => {
            log::error!("create stream error: {:?}", err);
            std::process::exit(1);
        }
    };

    log::info!("start subscribe");
    while let Some(response) = stream.next().await {
        match response {
            Ok(response) => match response.data {
                Some(data) => {
                    let mutation_type = match data.todos.mutation_type {
                        todo_changed::MutationType::CREATED => "created".to_string(),
                        todo_changed::MutationType::DELETED => "deleted".to_string(),
                        todo_changed::MutationType::Other(value) => value,
                    };
                    log::info!("subscribe response mutation_type: {:?}", mutation_type);
                    log::info!("subscribe response id: {:?}", data.todos.id);
                    let todo = data.todos.todo.unwrap();
                    log::info!(
                        "subscribe response todo: (id: {:?}, text: {})",
                        todo.id,
                        todo.text
                    );
                }
                None => {
                    log::info!("subscribe data is none");
                }
            },
            Err(err) => {
                log::error!("subscribe error: {:?}", err);
            }
        }
    }
    */

    // ココから下はgraphql-ws-clientのexampleのまま
    use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};
    use graphql_ws_client::Client;

    let mut request = "ws://localhost:3000/ws".into_client_request().unwrap();
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
    );

    let (connection, _) = async_tungstenite::async_std::connect_async(request)
        .await
        .unwrap();

    println!("Connected");

    let mut subscription = Client::build(connection)
        .subscribe(StreamingOperation::<TodoChanged>::new(
            todo_changed::Variables,
        ))
        .await
        .unwrap();

    log::info!("start subscribe");
    while let Some(response) = subscription.next().await {
        match response {
            Ok(response) => match response.data {
                Some(data) => {
                    let mutation_type = match data.todos.mutation_type {
                        todo_changed::MutationType::CREATED => "created".to_string(),
                        todo_changed::MutationType::DELETED => "deleted".to_string(),
                        todo_changed::MutationType::Other(value) => value,
                    };
                    log::info!("subscribe response mutation_type: {:?}", mutation_type);
                    log::info!("subscribe response id: {:?}", data.todos.id);
                    let todo = data.todos.todo.unwrap();
                    log::info!(
                        "subscribe response todo: (id: {:?}, text: {})",
                        todo.id,
                        todo.text
                    );
                }
                None => {
                    log::info!("subscribe data is none");
                }
            },
            Err(err) => {
                log::error!("subscribe error: {:?}", err);
            }
        }
    }
}
