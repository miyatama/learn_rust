use crate::TodoApiClient;

#[cfg_attr(feature = "mock", mockall::automock)]
pub trait DomainHandler {
    type TodoApi: TodoApiClient;
    fn todo_api_client(&self) -> &Self::TodoApi;
}
