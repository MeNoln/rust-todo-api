use postgres::{Client, NoTls};
use std::env;

pub fn get_dbconn() -> Client {
    let conn = env::var("DATABASE_URL").unwrap();
    return Client::connect(&conn, NoTls).unwrap();
}