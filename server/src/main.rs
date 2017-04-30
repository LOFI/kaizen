//! Kaizen Server is an HTTP service, mainly geared towards REST API interactions for a
//! variety of data, including user auth, registration, repo management, ACL, I don't know what
//! key management, etc.

#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate pwhash;
extern crate kaizen_core;

mod auth;
mod models;
mod schema;


fn main() {
    println!("Hello, world!");
}
