mod domain_handler;
mod domain_handler_impl;
mod domains;
mod domains_impl;

#[cfg_attr(feature = "mock", mockall_double::double)]
pub use domains::todo_api_client::TodoApiClient;
pub use domains_impl::todo_api_client_impl::TodoApiClientImpl;

#[cfg_attr(feature = "mock", mockall_double::double)]
pub use domain_handler::DomainHandler;
pub use domain_handler_impl::DomainHandlerImpl;
