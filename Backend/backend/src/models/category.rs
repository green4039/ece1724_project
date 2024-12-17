use diesel::prelude::*;
use crate::schema::categories;
use serde::{Deserialize, Serialize};

// Struct for querying users
// optional
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct Category {
    pub category_id: i32,
    pub email: String,
    pub nickname : String,
    pub category_type : String,
    pub budget: f64,
    pub budget_freq: String,
}

// Struct for inserting new users
#[derive(Insertable, Serialize, Deserialize, Clone)]
#[diesel(table_name = categories)]
pub struct NewCategory {
    pub email: String,
    pub nickname : String,
    pub category_type : String,
    pub budget: f64,
    pub budget_freq: String,
}
