use crate::domains::todo_api_client::TodoApiClient;
use util::AppResult;
use util::Todo;

#[derive(Debug, Clone)]
pub struct TodoApiClientImpl {}

impl Default for TodoApiClientImpl {
    fn default() -> Self {
        Self {}
    }
}

impl TodoApiClient for TodoApiClientImpl {
    fn list(&self) -> AppResult<Vec<Todo>> {
        // TODO ceate api call
        Ok(vec![])
    }
    fn create(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO ceate api call
        Err("todo call api")?
    }
    fn update(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO ceate api call
        Err("todo call api")?
    }
}
