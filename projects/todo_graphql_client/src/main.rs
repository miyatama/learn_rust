use reqwest::blocking::Client;

#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path="./src/schema.json",
    query_path="./src/query.graphql"
    normalization = "rust"
)]
pub struct TodoRepositories;

fn main() {
    println!("Hello, world!");
}

async fn perform_my_query() -> Result<(), Box<dyn Error>> {
    let variables = todo_repositories::Variables {
        after: LAST_ENTRY
            .lock()
            .ok()
            .and_then(|opt| opt.borrow().to_owned()),
    };

    let client = reqwest::blocking::Client::new();
    let response_body = graphql_client::reqwest::post_graphql_blocking::<TodoRepositories, _>(
        &client,
        "http://localhost:3000/",
        variables,
    )?;
    info!("{:?}", response_body);

    Ok(())
}
