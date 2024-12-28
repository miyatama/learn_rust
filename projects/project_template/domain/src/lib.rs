mod domains;
mod domains_impl;

pub use domains::todo_api_client::TodoApiClient;
pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;
use std::fmt::Debug;
use std::clone::Clone;

pub trait DomainHandler: Debug + Clone {
    type TodoApi: TodoApiClient;
    fn todo_api_client(&self) -> &Self::TodoApi;
}

#[derive(Debug, Clone)]
pub struct DomainHandlerImpl {
    todo_api_client: TodoApiClientImpl,
}

impl DomainHandler for DomainHandlerImpl {
    type TodoApi = TodoApiClientImpl;
    fn todo_api_client(&self) -> &Self::TodoApi {
        &self.todo_api_client
    }
}

impl DomainHandlerImpl {
    pub fn new() -> Self {
        Self {
            todo_api_client: TodoApiClientImpl::new(),
        }
    }
}
