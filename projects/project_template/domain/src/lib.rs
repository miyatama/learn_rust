mod domains;
mod domains_impl;

pub use domains::todo_api_client::TodoApiClient;
pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;

pub trait Domains {
    type TodoApiClient: TodoApiClient;
    fn todo_api_client(&self) -> &Self::TodoApiClient;
}

#[derive(Debug, Clone)]
pub struct DomainsImpl {
    todo_api_client: TodoApiClientImpl,
}

impl Domains for DomainsImpl {
    type TodoApiClient = TodoApiClientImpl;
    fn todo_api_client(&self) -> &Self::TodoApiClient {
        &self.todo_api_client
    }
}

impl DomainsImpl {
    pub fn new() -> Self {
        Self {
            todo_api_client: TodoApiClientImpl::new(),
        }
    }
}
