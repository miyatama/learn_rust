use crate::repositories::todo_repository::TodoRepository;
use util::AppResult;
use util::Todo;

#[derive(Debug, Clone)]
pub struct TodoRepositoryImpl {}

impl Default for TodoRepositoryImpl {
    fn default() -> Self {
        Self {}
    }
}

impl TodoRepository for TodoRepositoryImpl {
    fn create(&self) -> AppResult<Todo> {
        // TODO call domain
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
    fn update(&self, _todo: Todo) -> AppResult<Todo> {
        // TODO call domain
        Ok(Todo {
            id: 0,
            text: "".to_string(),
        })
    }
    fn list(&self) -> AppResult<Vec<Todo>> {
        // TODO call domain
        Ok(vec![])
    }
}
