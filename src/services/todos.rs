use crate::types::todo::{CreateTodoCommand, TodoResponse};
use crate::services::db::*;

pub trait ITodoService{
    fn new() -> Self;
    fn get_todos(&self, user_id: i32) -> Vec<TodoResponse>;
    fn get_todo(&self, user_id: i32, id: i32) -> Option<TodoResponse>;
    fn create_todo(&self, user_id: i32, todo: CreateTodoCommand) -> TodoResponse;
    fn delete_todo(&self, user_id: i32, todo_id: i32) -> bool;
}

pub struct TodoService{}

static GET_TODOS: &str = "select id, title, description, datecreated, completed from todos where user_id = $1;";
static GET_TODO_BY_ID: &str = "select id, title, description, datecreated, completed from todos where id = $1 and user_id = $2;";
static CREATE_TODO: &str = "insert into todos(user_id, title, description, datecreated, completed) values($1, $2, $3, $4, $5) returning id;";
static DELETE_TODO: &str = "delete from todos where user_id = $1 and id = $2";

impl ITodoService for TodoService {
    fn new() -> TodoService{return TodoService{}}

    fn get_todos(&self, user_id: i32) -> Vec<TodoResponse> {
        let mut conn = get_dbconn();
    
        let mut todos = Vec::new();
        for row in &conn.query(GET_TODOS, &[&user_id]).unwrap() {
            let todo = TodoResponse{
                id: row.get(0),
                title: row.get(1),
                description: row.get(2),
                datecreated: row.get(3),
                completed: row.get(4)
            };
    
            todos.push(todo);
        }
    
        let _ = conn.close();
        return todos;
    }
    
    fn get_todo(&self, user_id: i32, id: i32) -> Option<TodoResponse> {
        let mut conn = get_dbconn();
    
        let result = &conn.query(GET_TODO_BY_ID, &[&id, &user_id]).unwrap();
        if result.len() == 0 {
            return None;
        }
    
        let todo = TodoResponse{
            id: result[0].get(0),
            title: result[0].get(1),
            description: result[0].get(2),
            datecreated: result[0].get(3),
            completed: result[0].get(4)
        };
    
        let _ = conn.close();
        return Some(todo);
    }
    
    fn create_todo(&self, user_id: i32, todo: CreateTodoCommand) -> TodoResponse {
        let mut conn = get_dbconn();
    
        let inserted = &conn.query(CREATE_TODO, &[&user_id, &todo.title, &todo.description, &todo.datecreated, &todo.completed]).unwrap();
        let inserted_id: i32 = inserted[0].get(0);
    
        let _ = conn.close();
    
        return self.get_todo(user_id, inserted_id).unwrap();
    }
    
    fn delete_todo(&self, user_id: i32, todo_id: i32) -> bool {
        let mut conn = get_dbconn();
    
        let res = &conn.execute(DELETE_TODO, &[&user_id, &todo_id]).unwrap();
    
        let _ = conn.close();
    
        return *res == 1;
    }
}

// pub fn get_todos(user_id: i32) -> Vec<TodoResponse> {
//     let mut conn = get_dbconn();

//     let mut todos = Vec::new();
//     for row in &conn.query(GET_TODOS, &[&user_id]).unwrap() {
//         let todo = TodoResponse{
//             id: row.get(0),
//             title: row.get(1),
//             description: row.get(2),
//             datecreated: row.get(3),
//             completed: row.get(4)
//         };

//         todos.push(todo);
//     }

//     let _ = conn.close();
//     return todos;
// }



// pub fn get_todo(user_id: i32, id: i32) -> Option<TodoResponse> {
//     let mut conn = get_dbconn();

//     let result = &conn.query(GET_TODO_BY_ID, &[&id, &user_id]).unwrap();
//     if result.len() == 0 {
//         return None;
//     }

//     let todo = TodoResponse{
//         id: result[0].get(0),
//         title: result[0].get(1),
//         description: result[0].get(2),
//         datecreated: result[0].get(3),
//         completed: result[0].get(4)
//     };

//     let _ = conn.close();
//     return Some(todo);
// }



// pub fn create_todo(user_id: i32, todo: CreateTodoCommand) -> TodoResponse {
//     let mut conn = get_dbconn();

//     let inserted = &conn.query(CREATE_TODO, &[&user_id, &todo.title, &todo.description, &todo.datecreated, &todo.completed]).unwrap();
//     let inserted_id: i32 = inserted[0].get(0);

//     let _ = conn.close();

//     return get_todo(user_id, inserted_id).unwrap();
// }



// pub fn delete_todo(user_id: i32, todo_id: i32) -> bool {
//     let mut conn = get_dbconn();

//     let res = &conn.execute(DELETE_TODO, &[&user_id, &todo_id]).unwrap();

//     let _ = conn.close();

//     return *res == 1;
// }