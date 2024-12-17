use crate::schema::transactions;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

// Struct for querying transactions
// optional
#[derive(Debug, Queryable, Serialize, Deserialize, QueryableByName)]
pub struct Transaction {
    pub trans_id: i32,
    pub email: String,
    pub category_id: i32,
    pub amount: f64,
    pub notes: Option<String>,
    pub account_id: i32,
    pub transaction_date: String,
}

// Struct for inserting new transactions
#[derive(Insertable, Serialize, Deserialize, Queryable, QueryableByName)]
#[diesel(table_name = transactions)]
pub struct NewTransaction {
    pub email: String,
    pub category_id: i32,
    pub amount: f64,
    pub notes: Option<String>,
    pub account_id: i32,
    pub transaction_date: String,
}

// Struct for new transactions from client side
// Note that user is only aware of the names of accounts and categories
#[derive(Debug, Queryable, Serialize, Deserialize)]
pub struct ClientTransaction {
    pub email: String,
    pub category_name: String,
    pub amount: f64,
    pub notes: Option<String>,
    pub account_name: String,
}
