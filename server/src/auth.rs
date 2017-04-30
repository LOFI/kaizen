
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
