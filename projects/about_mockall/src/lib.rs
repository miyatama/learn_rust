mod logics;
use crate::logics::handlers::{GetClientHandler, LimitGetClientHandlerV1, LimitGetClientHandlerV2};
use crate::logics::repository_impl::InMemoryClientRepository;
use crate::logics::LimitInMemoryClientRepository;
use std::rc::Rc;

pub fn run() {
    let client_repository = InMemoryClientRepository::new();
    let _handler = GetClientHandler::new(Rc::new(client_repository));
    let client_repository2 = LimitInMemoryClientRepository::new(100usize);
    let _handler2 = LimitGetClientHandlerV1::new(Rc::new(client_repository2));
    let _handler3 = LimitGetClientHandlerV2::new();
}
