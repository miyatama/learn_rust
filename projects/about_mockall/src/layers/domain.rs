mod todo_client;
mod todo_client_impl;

pub use todo_client::TodoClient;
pub use todo_client_impl::TodoClientImpl;

pub trait DomainHandler {
    type TodoClientType: TodoClient;
    fn get_todo_client(&self) -> &Self::TodoClientType;
}

pub struct DomainHandlerImpl {
    todo_client: TodoClientImpl,
}

impl DomainHandlerImpl {
    pub fn new() -> Self {
        let todo_client = TodoClientImpl::new();
        Self {
            todo_client: todo_client,
        }
    }
}

impl DomainHandler for DomainHandlerImpl {
    type TodoClientType = TodoClientImpl;
    fn get_todo_client(&self) -> &Self::TodoClientType {
        &self.todo_client
    }
}
