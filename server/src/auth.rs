
use pwhash::{bcrypt, Result as HashResult};

use models::User;

fn check_auth(username: &str, password: &str, record: &User) -> bool {
    false
}

/// Generate a new hash for a plaintext password
fn hash_password(password: &str) -> HashResult<String> {
    bcrypt::hash(password)
}

/// Compares a unhashed password against a hash
fn check_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash)
}


pub mod handlers {
    use bodyparser;
    use iron::prelude::*;
    use iron::status;
    use persistent::Read;
    use router::Router;
    use serde_json;

    use models::NewUser;
    use super::hash_password;

    #[derive(Deserialize, Debug, Clone)]
    struct RegistrationData {
        pub email: String,
        pub display_name: String,
        pub username: String,
        pub password: String,
    }

    pub fn register(req: &mut Request) -> IronResult<Response> {
        let maybe_data = req.get::<bodyparser::Struct<RegistrationData>>();
        match maybe_data {
            Ok(Some(data)) => println!("Parsed body:\n{:?}", data),
            Ok(None) => println!("No body"),
            Err(err) => println!("Error: {:?}", err),
        }
        Ok(Response::with(status::Ok))
    }

    pub fn username_search(req: &mut Request) -> IronResult<Response> {
        let ref query = req.extensions.get::<Router>().unwrap().find("username").unwrap_or("/");

        Ok(Response::with(status::NotFound))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hash_password() {
        let hashed = hash_password("password").unwrap();

        assert!(check_password("password", hashed.as_ref()));

        let h = "$2y$05$bvIG6Nmid91Mu9RcmmWZfO5HJIMCT8riNW0hEp8f6/FuA2/mHZFpe";
        assert!(check_password("password", h));
    }

}
