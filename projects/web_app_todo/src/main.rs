mod util;

use self::util::app_logger::AppLogger;
use actix_web::{
    delete, get, http::header, post, put, web, App, HttpResponse, HttpServer, ResponseError,
};
use askama::Template;
use log::{debug, info, LevelFilter};
use r2d2::Pool;
use r2d2_sqlite::{SqliteConnectionManager};
use rusqlite::{params, Statement};
use serde::{Deserialize, Serialize};
use thiserror::Error;

static LOGGER: AppLogger = AppLogger {};

#[derive(Error, Debug)]
enum MyError {
    #[error("failed to render html")]
    AskamaError(#[from] askama::Error),

    #[error("failed to open db connection")]
    R2d2Error(#[from] r2d2::Error),

    #[error("failed sql execution")]
    SQLiteError(#[from] rusqlite::Error),
}

impl ResponseError for MyError {}

type RestResult = Result<HttpResponse, Box<dyn std::error::Error>>;

#[derive(Serialize, Deserialize, Debug)]
struct Todos {
    todos: Vec<TodoEntry>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct TodoEntry {
    id: u32,
    text: String,
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate {
    entries: Vec<TodoEntry>,
}

#[derive(Deserialize)]
struct GetTodosParams {
    text: String,
}

#[derive(Deserialize)]
struct AddTodosParams {
    text: String,
}

#[derive(Deserialize)]
struct UpdateTodoParams {
    id: u32,
    text: String,
}

#[derive(Deserialize)]
struct AddParams {
    text: String,
}

#[derive(Deserialize)]
struct DeleteParams {
    id: u32,
}

#[derive(Serialize)]
struct Memo {
    message: String,
}

#[get("/")]
async fn index(db: web::Data<Pool<SqliteConnectionManager>>) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id: id, text: text })
    })?;
    let mut entries = Vec::new();
    for row in rows {
        entries.push(row?);
    }
    let html = IndexTemplate { entries: entries };
    let response_body = html.render()?;
    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(response_body))
}

#[get("/todos")]
async fn get_todos(
    _params: web::Json<GetTodosParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> RestResult {
    info!("call /todos");
    let conn = db.get()?;
    let mut statement = conn.prepare("SELECT id, text FROM todo")?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id: id, text: text })
    })?;
    let mut todos = Vec::new();
    for row in rows {
        if row.is_ok() {
            todos.push(row.unwrap());
        }
    }
    debug!("/todos response: {:?}", todos);
    Ok(HttpResponse::Ok().json(Todos { todos: todos }))
}

#[post("/todo")]
async fn add_todo_rest(
    params: web::Json<AddTodosParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> RestResult {
    info!("call /todo");
    let conn = db.get()?;
    conn.execute("INSERT INTO todo(text) VALUES (?)", &[&params.text])?;
    let mut statement = conn.prepare(
        "SELECT T1.id, T1.text FROM todo T1 WHERE T1.id = (SELECT MAX(TMP.id) FROM todo TMP)",
    )?;
    let rows = statement.query_map(params![], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id: id, text: text })
    })?;
    let mut todos = Vec::new();
    for row in rows {
        if row.is_ok() {
            todos.push(row.unwrap());
            break;
        }
    }
    if todos.len() > 0 {
        Ok(HttpResponse::Ok().json(todos[0].clone()))
    } else {
        Err("todo not found after insert")?
    }
}

#[put("/todo")]
async fn update_todo_rest(
    params: web::Json<UpdateTodoParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> RestResult {
    info!("call /todo");
    let conn = db.get()?;
    conn.execute(
        "UPDATE todo SET text = ? WHERE id = ?",
        &[&params.text, &format!("{}", params.id)],
    )?;
    let mut statement = conn.prepare("SELECT T1.id, T1.text FROM todo T1 WHERE T1.id = ?").unwrap();
    let rows = statement.query_map(params![params.id], |row| {
        let id = row.get(0)?;
        let text = row.get(1)?;
        Ok(TodoEntry { id: id, text: text })
    })?;
    let mut todos = Vec::new();
    for row in rows {
        if row.is_ok() {
            todos.push(row.unwrap());
            break;
        }
    }
    if todos.len() > 0 {
        Ok(HttpResponse::Ok().json(todos[0].clone()))
    } else {
        Err("todo not found after update")?
    }
}

#[delete("/todo")]
async fn delete_todo_rest(
    params: web::Json<DeleteParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id = ?", &[&params.id])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[post("/add_todo")]
async fn add_todo(
    params: web::Form<AddParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("INSERT INTO todo(text) VALUES (?)", &[&params.text])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[post("/delete_todo")]
async fn delete_todo(
    params: web::Form<DeleteParams>,
    db: web::Data<r2d2::Pool<SqliteConnectionManager>>,
) -> Result<HttpResponse, MyError> {
    let conn = db.get()?;
    conn.execute("DELETE FROM todo WHERE id = ?", &[&params.id])?;
    Ok(HttpResponse::SeeOther()
        .header(header::LOCATION, "/")
        .finish())
}

#[post("/update_memo")]
async fn update_memo(params: web::Json<AddParams>) -> Result<HttpResponse, MyError> {
    Ok(HttpResponse::Ok().json(Memo {
        message: "Hello!".to_string(),
    }))
}

#[actix_rt::main]
async fn main() -> Result<(), actix_web::Error> {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(LevelFilter::Debug);
    let manager = SqliteConnectionManager::file("todo.db");
    let pool = Pool::new(manager).expect("failed to initialize the connection pool");
    let conn = pool
        .get()
        .expect("failed to get thee dconnection from the pool");
    conn.execute(
        "
        CREATE TABLE IF NOT EXISTS todo(
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            text TEXT NOT NULL
        )",
        params![],
    )
    .expect("failed to create talbe todo");
    HttpServer::new(move || {
        App::new()
            .service(index)
            .service(get_todos)
            .service(add_todo_rest)
            .service(delete_todo_rest)
            .service(update_todo_rest)
            .service(add_todo)
            .service(delete_todo)
            .service(update_memo)
            .data(pool.clone())
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await?;
    Ok(())
}
