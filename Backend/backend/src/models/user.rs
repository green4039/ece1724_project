use diesel::prelude::*;
use crate::schema::users;
use serde::{Deserialize, Serialize};

// Struct for querying users
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
    pub username: String,
}

// Struct for inserting new users
#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = users)]
pub struct NewUser {
    pub email: String,
    pub password: String,
    pub username: String,
}
