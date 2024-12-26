mod layers;
mod layers_trait_mock;
mod logics;

// UsecaseHandler
use crate::layers::{GetTodoUsecase, UsecaseHandler, UsecaseHandlerImpl};
use crate::layers_trait_mock::{
    GetTodo2Usecase, UsecaseHandler as TraitMockUsecaseHandler,
    UsecaseHandlerImpl as TraitMockUsecaseHandlerImpl,
};
use crate::logics::handlers::{GetClientHandler, LimitGetClientHandlerV1, LimitGetClientHandlerV2};
// GetClientHandler2
use crate::logics::InMemoryClientRepository;
use crate::logics::LimitInMemoryClientRepository;
use std::rc::Rc;

pub fn run() {
    let client_repository = InMemoryClientRepository::new();
    let _handler = GetClientHandler::new(Rc::new(client_repository));

    // ↓これはだめ
    /*
    let handler = GetClientHandler2::new();
    let repository = handler.get_repository();
     */

    let client_repository = LimitInMemoryClientRepository::new(100usize);
    let _handler = LimitGetClientHandlerV1::new(Rc::new(client_repository));
    let _handler = LimitGetClientHandlerV2::new();

    let handler = UsecaseHandlerImpl::new();
    let value = handler.get_todo_usecase().run();
    println!("result: {:?}", value);

    let handler = TraitMockUsecaseHandlerImpl::new();
    let value = handler.get_todo2_usecase().run();
    println!("result: {:?}", value);
}
