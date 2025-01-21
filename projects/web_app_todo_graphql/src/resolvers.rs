use async_graphql::{
    Context, ErrorExtensions, FieldError, FieldResult, Object, ResultExt, SimpleObject,
};

struct TodoRoot;

#[derive(SimpleObject)]
struct Ping {
    status: String,
    code: i32,
}

#[derive(SimpleObject)]
struct TodoData {
    id: u32,
    text: String,
}

#[derive(SimpleObject)]
struct TodoList {
    todos: Vec<TodoData>,
}

#[Object]
impl TodoRoot {
    async fn ping(&self) -> Ping {
        Ping {
            status: "ok".to_string(),
            code: 200,
        }
    }

    async fn get_todo(&self) -> TodoData {
        TodoData {
            id: 0,
            text: "hoge".to_string(),
        }
    }

    async fn get_todos(&self) -> TodoList {
        TodoList {
            todos: vec![
                TodoData {
                    id: 0,
                    text: "hoge".to_string(),
                },
                TodoData {
                    id: 1,
                    text: "huga".to_string(),
                },
            ],
        }
    }
}
