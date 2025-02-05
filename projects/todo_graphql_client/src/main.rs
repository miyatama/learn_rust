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

#[async_std::main]
async fn main() {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(log::LevelFilter::Debug);

    log::info!("start graphql client");

    let _ = perform_my_query();

    let mut request = "ws://localhost:8000/graphql".into_client_request().unwrap();
    use async_tungstenite::tungstenite::{client::IntoClientRequest, http::HeaderValue};
    request.headers_mut().insert(
        "Sec-WebSocket-Protocol",
        HeaderValue::from_str("graphql-transport-ws").unwrap(),
    );
    let (connection, _) = async_tungstenite::async_std::connect_async(request)
        .await
        .unwrap();
    use graphql_ws_client::graphql::StreamingOperation;
    let mut stream = graphql_ws_client::Client::build(connection)
        .subscribe(StreamingOperation::<TodoChanged>::new(
            todo_changed::Variables,
        ))
        .await?;
    while let Some(response) = stream.next().await {
        log::info!("subscribe response: {:?}", response);
    }

    // 終了マチ
    log::info!("any key to exit");
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    log::info!("finish graphql client");
}

fn perform_my_query() -> Result<(), Box<dyn std::error::Error>> {
    log::info!("start perform_my_query");

    // TODO追加
    let client = reqwest::blocking::Client::new();
    let variables = todo_update_repositories::Variables {
        text: Some("first todo from client".to_string()),
    };

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

    // 一覧取得
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

    Ok(())
}
