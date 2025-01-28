#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path="./src/schema.json",
    query_path="./src/query.graphql"
)]
pub struct TodoRepositories;

fn main() {
    println!("Hello, world!");
}

async fn perform_my_query(variables: union_query::Variables) -> Result<(), Box<dyn Error>> {

    // this is the important line
    let request_body = TodoRepositories::build_query(variables);

    let client = reqwest::Client::new();
    let mut res = client.post("/graphql").json(&request_body).send().await?;
    let response_body: Response<union_query::ResponseData> = res.json().await?;
    println!("{:#?}", response_body);
    Ok(())
}