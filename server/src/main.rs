//! Kaizen Server is an HTTP service, mainly geared towards REST API interactions for a
//! variety of data, including user auth, registration, repo management, ACL, I don't know what
//! key management, etc.

extern crate bodyparser;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate iron;
extern crate kaizen_core;
extern crate persistent;
extern crate pwhash;
#[macro_use]
extern crate router;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod auth;
mod models;
mod schema;

use dotenv::dotenv;
use iron::prelude::*;
use iron::status;
use persistent::Read;
use router::Router;
use std::env;

// pardon my dust - these hello handlers are from the examples for iron and iron router and I'm
// leaving them here until I figure out what I'm doing.

//fn hello_world(_: &mut Request) -> IronResult<Response> {
//    Ok(Response::with((status::Ok, "Hello World!")))
//}
//
//fn hello_blank(req: &mut Request) -> IronResult<Response> {
//    let ref query = req.extensions.get::<Router>().unwrap().find("blank").unwrap_or("/");
//    Ok(Response::with((status::Ok, *query)))
//}


const MAX_BODY_LENGTH: usize = 1024 * 1024 * 2;

fn main() {
    dotenv().ok();
    let prefix = match env::var("KAIZEN_API_ROOT") {
        Ok(url) => url,
        Err(_) => String::new(),
    };

    let router = router!(
        register: post format!("{}{}", prefix.as_str(), "/auth/register") =>
        auth::handlers::register,
        search_usernames: get format!("{}{}", prefix.as_str(), "/auth/username/:username") =>
            auth::handlers::username_search,

//        dynamic: get "/hello/:blank" => hello_blank,
//        hello: get "/hello" => hello_world,
//        index: get "/" => hello_world,

    );

    let mut chain = Chain::new(router);
    chain.link_before(Read::<bodyparser::MaxBodyLength>::one(MAX_BODY_LENGTH));

    // I'd make the port configurable via env, but it needs to line up with the proxy in the
    // frontend code, so I figure we'll pick and number and try to stick with it.
    let _ = Iron::new(chain).http("localhost:4500").unwrap();
    println!("Listening on 4500");
}
