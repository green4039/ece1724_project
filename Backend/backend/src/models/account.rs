use diesel::prelude::*;
use crate::schema::accounts;
use serde::{Deserialize, Serialize};

// Struct for querying users
// optional
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Account {
    pub account_id: i32,
    pub email: String,
    pub account_type: String,
    pub account_name: String,
}

// Struct for inserting new users
#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = accounts)]
pub struct NewAccount {
    pub email: String,
    pub account_type: String,
    pub account_name: String,
}
