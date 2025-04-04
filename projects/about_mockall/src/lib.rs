mod associated_type;
mod call_counts;
mod checkpoints;
mod generics;
mod getting_started;
mod impl_future;
mod impl_trait;
mod layers;
mod layers_trait_mock;
mod logics;
mod matching_multiple_values;
mod mocking_structs;
mod references;
mod sequence;
mod static_return_values;

// UsecaseHandler
use crate::layers::{GetTodoUsecase, UsecaseHandler, UsecaseHandlerImpl};
use crate::layers_trait_mock::{
    GetTodo2Usecase, RepositoryHandlerImpl as TraitMockRepositoryHanlderImpl,
    UsecaseHandler as TraitMockUsecaseHandler, UsecaseHandlerImpl as TraitMockUsecaseHandlerImpl,
};
use crate::logics::handlers::{GetClientHandler, LimitGetClientHandlerV1, LimitGetClientHandlerV2};
// GetClientHandler2
use crate::logics::InMemoryClientRepository;
use crate::logics::LimitInMemoryClientRepository;
use std::rc::Rc;

use crate::associated_type::call_associated_type;
use crate::call_counts::call_call_counts;
use crate::checkpoints::call_checkpoints;
use crate::generics::call_generics;
use crate::getting_started::call_getting_started_func;
use crate::impl_future::call_impl_future;
use crate::impl_trait::call_impl_trait;
use crate::matching_multiple_values::call_matching_multiple_values;
use crate::mocking_structs::call_mocking_structs;
use crate::references::call_references;
use crate::sequence::call_sequence;
use crate::static_return_values::call_static_return_values;

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

    let repository_handler = TraitMockRepositoryHanlderImpl::new();
    let handler = TraitMockUsecaseHandlerImpl::new(&repository_handler);
    let value = handler.get_todo2_usecase().run();
    println!("result: {:?}", value);

    call_getting_started_func();
    call_static_return_values();
    call_matching_multiple_values();
    call_call_counts();
    call_checkpoints();
    call_sequence();
    call_references();
    call_impl_trait();
    call_impl_future();
    call_mocking_structs();
    call_generics();
    call_associated_type();
}
