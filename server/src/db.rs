use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenv::dotenv;
use std::env;
use super::models::{User, NewUser};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}



pub fn create_user<'a>(conn: &PgConnection, title: &'a str, body: &'a str) -> User {
    use schema::user;

    let user = NewUser {
        title: title,
        body: body,
    };

    diesel::insert(&user).into(user::table)
        .get_result(conn)
        .expect("Error saving new user")
}
