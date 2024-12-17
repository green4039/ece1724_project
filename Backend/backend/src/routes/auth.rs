use rocket::serde::json::Json;
use rocket::State;
use rocket::http::Status;
use crate::models::user::{NewUser};
use crate::db::DbPool;
use crate::handlers::auth_handler;

#[post("/signup", format = "json", data = "<new_user>")]
pub async fn signup(new_user: Json<NewUser>, pool: &State<DbPool>) -> (Status, &'static str) {
    auth_handler::handle_signup(new_user.into_inner(), pool.inner().clone()).await
}
