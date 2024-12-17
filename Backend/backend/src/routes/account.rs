use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use crate::models::account::NewAccount;
use crate::db::DbPool;
use crate::handlers::account_handler;
use rocket::form::FromForm;
use crate::models::account::Account;
#[allow(unused_imports)]
use rocket::serde::Serialize;

#[post("/account_create", format = "json", data = "<new_acc>")]
pub async fn account_create(new_acc: Json<NewAccount>, pool: &State<DbPool>) -> (Status, String) {
    account_handler::handle_account_create(new_acc.into_inner(), pool.inner().clone()).await
}

// A struct to parse the query parameter
#[derive(FromForm)]
pub struct AccountQuery {
    pub email: String,
}

// GET route that uses a query parameter
#[get("/account_summary?<account_query..>")]
pub async fn account_summary(account_query: AccountQuery, pool: &State<DbPool>) -> (Status, Json<Vec<Account>>) {
    account_handler::handle_account_summary(account_query.email, pool.inner().clone()).await
}

// For /delete_account
#[derive(FromForm)]
pub struct DeleteAccountQuery {
    pub email: String,
    pub account_name: String,
}

// DELETE
#[delete("/delete_account?<delete_query..>")]
pub async fn delete_account(delete_query: DeleteAccountQuery, pool: &State<DbPool>) -> (Status, &'static str) {
    account_handler::handle_delete_account(delete_query.email, delete_query.account_name, pool.inner().clone()).await
}