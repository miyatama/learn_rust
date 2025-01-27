#[derive(graphql_client::GraphQLQuery)]
#[graphql(
    schema_path="./schema.json",
)]
pub struct TodoQuery;

fn main() {
    println!("Hello, world!");
}
