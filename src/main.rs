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
mod middleware;

use types::todo::{CreateTodoCommand, TodoResponse};
use types::user::{UserProdileResponse, UserIdResponse};
use types::error::Error;

fn ping(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Ping ok")))
}

fn login(r: &mut Request) -> IronResult<Response> {
    let uname: &str = r.extensions.get::<Router>().unwrap().find("username").unwrap_or("");
    println!("Got username: {}", uname);

    let user = services::users::login_user(uname);
    Ok(format_response::<UserIdResponse>(user, iron::status::Ok))
}

fn get_profile(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(middleware::authmiddleware::USERID_HEADER));

    let user = services::users::get_user(user_id);
    Ok(format_response::<UserProdileResponse>(user, iron::status::Ok))
}

fn get_todos(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(middleware::authmiddleware::USERID_HEADER));
    
    let todos = services::todos::get_todos(user_id);
    Ok(format_response::<Vec<TodoResponse>>(todos, iron::status::Ok))
}

fn get_todo(r: &mut Request) -> IronResult<Response> {
    let user_id = get_user_header(r.headers.get_raw(middleware::authmiddleware::USERID_HEADER));

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
    let user_id = get_user_header(r.headers.get_raw(middleware::authmiddleware::USERID_HEADER));
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

    let a = std::env::var("DATABASE_URL").unwrap();
    println!("{}", a);

    let mut r = Router::new();
    
    //public
    r.get("/ping", ping, "ping");
    r.get("/api/auth/:username", login, "login");

    //authenticated
    let mut get_profile_chain = Chain::new(get_profile);
    get_profile_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.get("/api/profile", get_profile_chain, "profile");

    let mut get_todos_chain = Chain::new(get_todos);
    get_todos_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.get("/api/todo", get_todos_chain, "all");

    let mut get_todo_chain = Chain::new(get_todo);
    get_todo_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.get("/api/todo/:id", get_todo_chain, "current");

    let mut create_todo_chain = Chain::new(create_todo);
    create_todo_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.post("/api/todo", create_todo_chain, "create");

    let mut update_todo_chain = Chain::new(update_todo);
    update_todo_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.put("/api/todo/:id", update_todo_chain, "update");

    let mut delete_todo_chain = Chain::new(delete_todo);
    delete_todo_chain.link_before(middleware::authmiddleware::AuthorizationMiddleware);
    r.delete("/api/todo/:id", delete_todo_chain, "delete");

    println!("Start");
    Iron::new(r).http("0.0.0.0:5000").unwrap();
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
            return val.parse().expect("0");
        },
        None => return 0
    }
}