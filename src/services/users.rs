use crate::types::user::{UserIdResponse, User};
use crate::services::db::*;

static GET_USER: &str = "select id, name from users where name = $1";
static INSERT_USER_RETURNING_ID: &str = "insert into users(name) values ($1) returning id;";

pub fn login_user(username: &str) -> UserIdResponse {
    let mut conn = get_dbconn();

    let query = &conn.query_one(GET_USER, &[&username]);
    if query.is_err() {
        let new_user_id = &conn.query_one(INSERT_USER_RETURNING_ID, &[&username]).unwrap();
        return UserIdResponse{id: new_user_id.get(0)};
    }
    
    let row = query.as_ref().unwrap();
    let user = User{
        id: row.get(0),
        name: row.get(1)
    };

    let _ = conn.close();
    return UserIdResponse{id: user.id};
}

static GET_USER_BY_ID: &str = "select id from users where id = $1;";

pub fn check_user_exists(user_id: i32) -> bool {
    let mut conn = get_dbconn();
    
    let exists = &conn.query(GET_USER_BY_ID, &[&user_id]).unwrap();
    let len = exists.len();

    let _ = conn.close();
    return len != 0;
}