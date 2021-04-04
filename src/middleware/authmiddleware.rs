use iron::prelude::*;
use iron::{BeforeMiddleware};
use std::fmt::{self, Debug};
use std::error::Error;

use crate::services::users;

static TOKEN_HEADER: &str = "X-User-Id";
pub static USERID_HEADER: &str = "UserId";

pub struct AuthorizationMiddleware;
impl BeforeMiddleware for AuthorizationMiddleware {
    fn before(&self, r: &mut Request) -> IronResult<()> {
        let raw_token = r.headers.get_raw(TOKEN_HEADER);
        match raw_token {
            Some(token_b) => {
                let token = String::from_utf8(token_b[0].clone()).unwrap();
                println!("Trying to access route with token: {}", token);

                let exists = users::check_user_exists(token);
                if !exists.0 {
                    println!("User with token not found.");
                    return return_unauth();
                }

                let user_id = exists.1.to_string();
                println!("Accessing user with id: {}", user_id);

                r.headers.set_raw(USERID_HEADER, vec![user_id.as_bytes().to_vec()]);
                Ok(())
            }
            None => {
                println!("Unauthorized access.");
                return_unauth()
            }
        }
    }
}

fn return_unauth() -> IronResult<()> {
    return Err(IronError::new(StringError("".to_string()), iron::status::Unauthorized))
}

#[derive(Debug)]
struct StringError(String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str { &*self.0 }
}