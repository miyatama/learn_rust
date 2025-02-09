use crate::models::todo::Todo;
use crate::models::todo_service_data::TodoServiceData;
use futures_util::{lock::Mutex, StreamExt};
use std::sync::Arc;

pub type Storage = Arc<Mutex<TodoServiceData>>;

pub struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
    async fn todos<'a>(&self, ctx: &async_graphql::Context<'a>) -> Vec<Todo> {
        log::info!("QueryRoot::todos()");
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
        log::info!("MutationRoot::add_todo()");
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
        TodoBroker::publish(TodoChanged {
            mutation_type: MutationType::Created,
            id: id,
        });
        Ok(id)
    }

    async fn delete_todo(
        &self,
        ctx: &async_graphql::Context<'_>,
        id: u32,
    ) -> async_graphql::Result<u32> {
        log::info!("MutationRoot::delete_todo()");
        let todo_service_data = &mut ctx.data_unchecked::<Storage>().lock().await;
        let todos = &mut todo_service_data.todos;
        let index = todos.iter().position(|todo| todo.id == id).unwrap();
        todos.remove(index);
        TodoBroker::publish(TodoChanged {
            mutation_type: MutationType::Deleted,
            id: id,
        });
        Ok(id)
    }
}

#[derive(async_graphql::Enum, Eq, PartialEq, Copy, Clone)]
enum MutationType {
    Created,
    Deleted,
}

#[derive(Clone)]
pub struct TodoChanged {
    mutation_type: MutationType,
    id: u32,
}

#[async_graphql::Object]
impl TodoChanged {
    async fn mutation_type(&self) -> MutationType {
        self.mutation_type
    }

    async fn id(&self) -> u32 {
        *(&self.id)
    }

    async fn todo(&self, ctx: &async_graphql::Context<'_>) -> async_graphql::Result<Option<Todo>> {
        log::info!("TodoChanged::todo()");
        let todo_service_data = &ctx.data_unchecked::<Storage>().lock().await;
        let todos = todo_service_data.todos().clone();
        let todo = todos.iter().filter(|todo| todo.id == self.id).next();
        let todo = if let Some(todo) = todo {
            Some(todo.clone())
        } else {
            None
        };
        Ok(todo)
    }
}

use crate::models::todo_broker::TodoBroker;
pub struct SubscriptionRoot;

#[async_graphql::Subscription]
impl SubscriptionRoot {
    async fn interval(
        &self,
        #[graphql(default = 1)] n: i32,
    ) -> impl futures_util::Stream<Item = i32> {
        log::info!("SubscriptionRoot::interval()");
        let mut value = 0;
        async_stream::stream! {
            loop {
                futures_timer::Delay::new(std::time::Duration::from_secs(1)).await;
                value += n;
                yield value;
            }
        }
    }

    async fn todos(
        &self,
        mutation_type: Option<MutationType>,
    ) -> impl futures_util::Stream<Item = TodoChanged> {
        log::info!("SubscriptionRoot::todos()");
        TodoBroker::<TodoChanged>::subscribe().filter(move |event| {
            let res = if let Some(mutation_type) = mutation_type {
                event.mutation_type == mutation_type
            } else {
                true
            };
            async move { res }
        })
    }
}
