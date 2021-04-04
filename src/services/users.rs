use sha256::digest;

use crate::types::user::{UserIdResponse, User, UserProdileResponse};
use crate::services::db::*;

static GET_USER: &str = "select id, name, token from users where name = $1";
static INSERT_USER_RETURNING_ID: &str = "insert into users(name, token) values ($1, $2) returning token;";

pub fn login_user(username: &str) -> UserIdResponse {
    let mut conn = get_dbconn();

    let query = &conn.query_one(GET_USER, &[&username]);
    if query.is_err() {
        let token = generate_token(username);

        let new_user_id = &conn.query_one(INSERT_USER_RETURNING_ID, &[&username, &token]).unwrap();
        let id: String = new_user_id.get(0);

        let _ = conn.close();

        return UserIdResponse{id: id};
    }
    
    let row = query.as_ref().unwrap();
    let user = User{
        id: row.get(0),
        name: row.get(1),
        token: row.get(2)
    };

    let _ = conn.close();
    return UserIdResponse{id: user.token};
}

static GET_USER_BY_TOKEN: &str = "select id from users where token = $1;";

pub fn check_user_exists(token: String) -> (bool, i32) {
    let mut conn = get_dbconn();
    
    let exists = &conn.query(GET_USER_BY_TOKEN, &[&token]).unwrap();
    if exists.len() == 0 {
        return (false, 0);
    }

    let user_id: i32 = exists[0].get(0);

    let _ = conn.close();

    return (true, user_id);
}

fn generate_token(name: &str) -> String {
    return digest(name);
}

static GET_USER_BY_ID: &str = "select name from users where id = $1;";

pub fn get_user(id: i32) -> UserProdileResponse {
    let mut conn = get_dbconn();
    
    let user = &conn.query_one(GET_USER_BY_ID, &[&id]).unwrap();
    let profile = UserProdileResponse{
        name: user.get(0)
    };

    let _ = conn.close();
    return profile;
}