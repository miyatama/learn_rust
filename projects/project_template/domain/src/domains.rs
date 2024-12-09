use crate::todo_api::todo_api_client::TodoApiClient;

pub trait Domains{
    type TodoApiClient: TodoApiClient;
    fn todo_api_client(&self) -> &Self::TodoApiClient;
}