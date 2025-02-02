use crate::models::todo::Todo;
use crate::models::todo_service_data::TodoServiceData;
use futures_util::lock::Mutex;
use std::sync::Arc;

pub type Storage = Arc<Mutex<TodoServiceData>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos<'a>(&self, ctx: &async_graphql::Context<'a>) -> Vec<Todo> {
        let todo_service_data = &ctx.data_unchecked::<Storage>().lock().await;
        todo_service_data.todos()
    }
}

pub struct MutationRoot;

#[async_graphql::Object]
impl MutationRoot {
    async fn add_todo(
        &self,
        ctx: &async_graphql::Context<'_>,
        text: String,
    ) -> async_graphql::Result<u32> {
        let todo_service_data = &mut ctx.data_unchecked::<Storage>().lock().await;
        let todos = &mut todo_service_data.todos;
        let id = match todos.iter().max_by_key(|todo| todo.id) {
            Some(todo) => todo.id + 1,
            None => 1,
        };
        todos.push(Todo {
            id: id,
            text: text.clone(),
        });
        Ok(id)
    }
}
