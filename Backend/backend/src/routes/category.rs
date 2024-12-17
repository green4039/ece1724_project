use crate::db::DbPool;
use crate::handlers::category_handler;
use crate::models::category::Category;
use crate::models::category::NewCategory;
use rocket::http::Status;
use rocket::serde::json::Json;
#[allow(unused_imports)]
use rocket::serde::Serialize;
use rocket::State;

#[post("/category_create", format = "json", data = "<new_cat>")]
pub async fn category_create(new_cat: Json<NewCategory>, pool: &State<DbPool>) -> (Status, String) {
    category_handler::handle_category_create(new_cat.into_inner(), pool.inner().clone()).await
}

// For /delete_category
#[derive(FromForm)]
pub struct DeleteCategoryQuery {
    pub email: String,
    pub category_nickname: String,
}

// DELETE
#[delete("/delete_category?<delete_query..>")]
pub async fn delete_category(
    delete_query: DeleteCategoryQuery,
    pool: &State<DbPool>,
) -> (Status, &'static str) {
    category_handler::handle_delete_category(
        delete_query.email,
        delete_query.category_nickname,
        pool.inner().clone(),
    )
    .await
}

// A struct to parse the query parameter
#[derive(FromForm)]
pub struct CategoryQuery {
    pub email: String,
}

// GET route that uses a query parameter
#[get("/category_summary?<category_query..>")]
pub async fn category_summary(
    category_query: CategoryQuery,
    pool: &State<DbPool>,
) -> (Status, Json<Vec<Category>>) {
    category_handler::handle_category_summary(category_query.email, pool.inner().clone()).await
}

// A struct to parse the query parameter
#[derive(FromForm)]
pub struct CategoryUpdateQuery {
    pub email: String,
    pub field: String,
    pub category_nickname: String,
    pub new_value: String,
}

// POST route that uses a query parameter
#[post("/category_update?<update_query..>")]
pub async fn category_update(
    update_query: CategoryUpdateQuery,
    pool: &State<DbPool>,
) -> (Status, String) {
    category_handler::handle_category_update(
        update_query.email,
        update_query.field,
        update_query.category_nickname,
        update_query.new_value,
        pool.inner().clone(),
    )
    .await
}
