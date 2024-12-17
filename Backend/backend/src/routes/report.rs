use crate::db::DbPool;
use crate::handlers::report_handler;
use report_handler::CategorySummary;
use rocket::http::Status;
use rocket::serde::json::Json;
#[allow(unused_imports)]
use rocket::serde::Serialize;
use rocket::State;

// For /report_overview
#[derive(FromForm)]
pub struct OverviewQuery {
    pub email: String,
}

// GET
#[get("/report_overview?<overview_query..>")]
pub async fn report_overview(
    overview_query: OverviewQuery,
    pool: &State<DbPool>,
) -> (Status, Json<Vec<String>>) {
    report_handler::handle_report_overview(overview_query.email, pool.inner().clone()).await
}

// For /report_details
#[derive(FromForm)]
pub struct DetailsQuery {
    pub email: String,
}

// GET
#[get("/report_details?<details_query..>")]
pub async fn report_details(
    details_query: DetailsQuery,
    pool: &State<DbPool>,
) -> (Status, Json<Vec<CategorySummary>>) {
    report_handler::handle_report_details(details_query.email, pool.inner().clone()).await
}
