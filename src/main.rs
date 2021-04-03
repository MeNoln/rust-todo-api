extern crate router;
extern crate iron;
extern crate serde;
extern crate dotenv;
extern crate postgres;
extern crate bodyparser;

#[macro_use]
extern crate serde_json;

#[macro_use]
extern crate serde_derive;


use router::Router;
use iron::prelude::*;
use iron::mime::Mime;
use serde::ser::Serialize;
use dotenv::dotenv;


mod types;
mod services;

use types::todo::{Todo, CreateTodoCommand, TodoResponse};
use types::user::UserIdResponse;
use types::error::Error;

static USERID_HEADER: &str = "X-User-Id"; 

fn login(r: &mut Request) -> IronResult<Response> {
    let uname: &str = r.extensions.get::<Router>().unwrap().find("username").unwrap_or("");
    println!("Got username: {}", uname);

    let user = services::users::login_user(uname);
    Ok(format_response::<UserIdResponse>(user, iron::status::Ok))
}

fn get_todos(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(USERID_HEADER));
    
    let todos = services::todos::get_todos(user_id);
    Ok(format_response::<Vec<TodoResponse>>(todos, iron::status::Ok))
}

fn get_todo(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(USERID_HEADER));

    let ref todo_id_str = r.extensions.get::<Router>().unwrap().find("id").unwrap_or("0");
    let todo_id: i32 = todo_id_str.parse().expect("Wanted a number");
    println!("Searching for todo with id: {}", todo_id);

    let todo = services::todos::get_todo(user_id, todo_id);
    if todo.is_some() {
        let raw = todo.unwrap();
        return Ok(format_response::<TodoResponse>(raw, iron::status::Ok))
    }
    else {
        return Ok(Response::with(iron::status::NotFound))
    }
}

fn create_todo(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(USERID_HEADER));
    let body = r.get::<bodyparser::Struct<CreateTodoCommand>>();

    let todo: CreateTodoCommand;
    match body {
        Ok(Some(body)) => todo = body,
        Ok(None) => return Ok(format_response::<Error>(Error{message: String::from("Failed to parse body")}, iron::status::BadRequest)),
        Err(err) => return Ok(format_response::<Error>(Error{message: err.to_string()}, iron::status::BadRequest))
    }
    
    let todo = services::todos::create_todo(user_id, todo);
    Ok(format_response::<TodoResponse>(todo, iron::status::Ok))
}

fn update_todo(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello")))
}

fn delete_todo(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello")))
}

fn main() {
    dotenv().ok();

    let mut r = Router::new();
    r.get("/api/auth/:username", login, "login");
    r.get("/api/todo", get_todos, "all");
    r.get("/api/todo/:id", get_todo, "current");
    r.post("/api/todo", create_todo, "create");
    r.put("/api/todo/:id", update_todo, "update");
    r.delete("/api/todo/:id", delete_todo, "delete");

    println!("Starting server at :5000");
    Iron::new(r).http("0.0.0.0:5000");
}


fn format_response<T: Serialize>(body: T, status: iron::status::Status) -> Response {
    let content_type = "application/json".parse::<Mime>().unwrap();
    let payload = json!(body).to_string();

    return Response::with((content_type, status, payload))
}

fn get_user_header(header: Option<&[Vec<u8>]>) -> i32 {
    match header {
        Some(header) => {
            let val = String::from_utf8(header[0].clone()).unwrap();
            return val.parse().expect("");
        },
        None => return 0
    }
}