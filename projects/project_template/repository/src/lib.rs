mod repositories;

pub use repositories::todo_repository::TodoRepository;

pub trait Repositories {
    type TodoRepository: TodoRepository;
    fn todo_repository(&self) -> &Self::TodoRepository;
}
