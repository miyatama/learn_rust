mod logics;
use crate::logics::handlers::{GetClientHandler2, GetClientHandler, LimitGetClientHandlerV1, LimitGetClientHandlerV2};
use crate::logics::repository_impl::InMemoryClientRepository;
use crate::logics::LimitInMemoryClientRepository;
use std::rc::Rc;

pub fn run() {
    let client_repository = InMemoryClientRepository::new();
    let handler = GetClientHandler::new(Rc::new(client_repository));

    // ↓これはだめ
    /*
    let handler = GetClientHandler2::new();
    let repository = handler.get_repository();
     */

    let client_repository = LimitInMemoryClientRepository::new(100usize);
    let handler = LimitGetClientHandlerV1::new(Rc::new(client_repository));
    let handler = LimitGetClientHandlerV2::new();
}
