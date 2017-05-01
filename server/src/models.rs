use super::schema::users;

/// User represents a db record for a user account.
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub display_name: String,
    pub username: String,
    pub hasher: String,
    pub password: String,
    pub email_verified: bool,
    pub staff: bool,
    pub admin: bool,
    pub active: bool,
}

/// NewUser is for collecting user input then inserting into the DB.
#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub display_name: &'a str,
    pub username: &'a str,
    pub hasher: &'a str,
    pub password: &'a str,
    pub email_verified: bool,
    pub staff: bool,
    pub admin: bool,
}
