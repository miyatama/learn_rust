mod todo2_client;
mod todo2_client_impl;

pub use todo2_client::Todo2Client;
pub use todo2_client_impl::Todo2ClientImpl;

pub trait DomainHandler {
    type Todo2ClientType: Todo2Client;
    fn get_todo2_client(&self) -> &Self::Todo2ClientType;
}

pub struct DomainHandlerImpl {
    todo2_client: Todo2ClientImpl,
}

impl DomainHandlerImpl {
    pub fn new() -> Self {
        let todo2_client = Todo2ClientImpl::new();
        Self {
            todo2_client: todo2_client,
        }
    }
}

impl DomainHandler for DomainHandlerImpl {
    type Todo2ClientType = Todo2ClientImpl;
    fn get_todo2_client(&self) -> &Self::Todo2ClientType {
        &self.todo2_client
    }
}
