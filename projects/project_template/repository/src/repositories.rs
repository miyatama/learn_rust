mod todo_repository;

#[cfg(feature = "mock")]
use mockall_double::double;

#[cfg_attr(feature = "mock", double)]
pub use todo_repository::TodoRepository;