use crate::db::DbPool;
use crate::models::category::{Category, NewCategory};
use crate::models::user::User;
use crate::schema::categories::dsl::*;
use crate::schema::users::dsl::{email as user_email, users}; // For users table
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;

pub async fn handle_category_create(new_cat: NewCategory, pool: DbPool) -> (Status, String) {
    // Step 1: Validate input
    if new_cat.email.is_empty()
        || new_cat.category_type.is_empty()
        || new_cat.nickname.is_empty()
        || new_cat.budget_freq.is_empty()
    {
        return (Status::BadRequest, "Invalid input".to_string());
    }

    // Step 1.5: Check if the email exists in users table
    // If not, no category should be created
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_cat.email.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users
                .filter(user_email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    })
    .await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (
                Status::BadRequest,
                "No user found for the provided email".to_string(),
            );
        }
        Ok(Ok(Some(_user))) => {
            // User found, proceed to category name existence check
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (
                Status::InternalServerError,
                "Internal server error".to_string(),
            );
        }
    }

    // Step 2: Check if the category nickname already exists for the given email
    let nickname_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_cat.email.clone();
        let name_to_check = new_cat.nickname.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(email.eq(email_to_check))
                .filter(nickname.eq(name_to_check))
                .first::<Category>(&mut conn)
                .optional()
        }
    })
    .await;

    match nickname_exists {
        Ok(Ok(Some(_existing_nickname))) => {
            // Category nickname already taken for this email
            return (
                Status::BadRequest,
                "Failed to create new category: duplicate nicknames".to_string(),
            );
        }
        Ok(Ok(None)) => {
            // Step 3: Proceed to create the new category
            let cat_to_insert = new_cat.clone(); // Clone the NewCategory to avoid moving it
            let nickname_for_message = new_cat.clone(); // Another clone to access nickname later
            let result = tokio::task::spawn_blocking({
                let pool = pool.clone();
                move || {
                    let mut conn = pool.get().expect("Failed to get database connection");
                    diesel::insert_into(categories)
                        .values(&cat_to_insert) // Use one copy of the NewCategory
                        .execute(&mut conn)
                }
            })
            .await;

            match result {
                Ok(Ok(_)) => {
                    // Successfully inserted the category
                    let msg = format!("Successfully created {}", nickname_for_message.nickname);
                    (Status::Created, msg)
                }
                Ok(Err(e)) => {
                    eprintln!("Database error during insertion: {:?}", e);
                    (Status::InternalServerError, "Database error".to_string())
                }
                Err(e) => {
                    eprintln!("Blocking task failed during insertion: {:?}", e);
                    (
                        Status::InternalServerError,
                        "Internal server error".to_string(),
                    )
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error checking category existence: {:?}", e);
            (Status::InternalServerError, "Database error".to_string())
        }
        Err(e) => {
            eprintln!("Blocking task failed during category check: {:?}", e);
            (
                Status::InternalServerError,
                "Internal server error".to_string(),
            )
        }
    }
}

// DELETE delete category
pub async fn handle_delete_category(
    email_str: String,
    category_nickname: String,
    pool: DbPool,
) -> (Status, &'static str) {
    // Check if email is empty or category_nickname is empty
    if email_str.is_empty() || category_nickname.is_empty() {
        return (Status::BadRequest, "Invalid input");
    }

    // Check if user exists
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users
                .filter(user_email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    })
    .await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (Status::BadRequest, "No user found for the provided email");
        }
        Ok(Ok(Some(_))) => {
            // User found, proceed with category deletion
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error");
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (Status::InternalServerError, "Internal server error");
        }
    }

    // Check if the category nickname exists for this user
    let category_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
        let nickname_to_check = category_nickname.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(email.eq(email_to_check))
                .filter(nickname.eq(nickname_to_check))
                .first::<Category>(&mut conn)
                .optional()
        }
    })
    .await;

    let found_category = match category_exists {
        Ok(Ok(Some(cat))) => cat,
        Ok(Ok(None)) => {
            // category not found for this email
            return (
                Status::BadRequest,
                "No such category found for the provided email",
            );
        }
        Ok(Err(e)) => {
            eprintln!("Error checking category existence: {:?}", e);
            return (Status::InternalServerError, "Database error");
        }
        Err(e) => {
            eprintln!(
                "Blocking task failed during category existence check: {:?}",
                e
            );
            return (Status::InternalServerError, "Internal server error");
        }
    };

    // Proceed to delete the found category
    let deletion_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let to_delete_id = found_category.category_id;
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            diesel::delete(categories.filter(category_id.eq(to_delete_id))).execute(&mut conn)
        }
    })
    .await;

    match deletion_result {
        Ok(Ok(rows_deleted)) => {
            if rows_deleted > 0 {
                (Status::Ok, "Category successfully deleted")
            } else {
                (Status::InternalServerError, "Failed to delete the category")
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error during deletion: {:?}", e);
            (
                Status::InternalServerError,
                "Database error during deletion",
            )
        }
        Err(e) => {
            eprintln!("Blocking task failed during deletion: {:?}", e);
            (Status::InternalServerError, "Internal server error")
        }
    }
}

// GET /category_summary?email=<>
pub async fn handle_category_summary(
    email_str: String,
    pool: DbPool,
) -> (Status, Json<Vec<Category>>) {
    // If email is empty, return bad request
    if email_str.is_empty() {
        return (Status::BadRequest, Json(vec![]));
    }

    let categories_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_search = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(email.eq(email_to_search))
                .load::<Category>(&mut conn)
        }
    })
    .await;

    match categories_result {
        Ok(Ok(cat_list)) => {
            // Successfully retrieved categories
            (Status::Ok, Json(cat_list))
        }
        Ok(Err(e)) => {
            eprintln!("Database error during category summary retrieval: {:?}", e);
            (Status::InternalServerError, Json(vec![]))
        }
        Err(e) => {
            eprintln!(
                "Blocking task failed during category summary retrieval: {:?}",
                e
            );
            (Status::InternalServerError, Json(vec![]))
        }
    }
}

// POST /category_update?email=<user_email>&field=<field_to_update>&category_nickname=<nickname>&new_value=<new_value>
use std::str::FromStr;
#[derive(Debug, PartialEq)]
enum ValidCategoryFields {
    Name,
    Type,
    Budget,
    Freq,
}

impl FromStr for ValidCategoryFields {
    type Err = ();
    fn from_str(input: &str) -> Result<ValidCategoryFields, Self::Err> {
        match input {
            "nickname" => Ok(ValidCategoryFields::Name),
            "category_type" => Ok(ValidCategoryFields::Type),
            "budget" => Ok(ValidCategoryFields::Budget),
            "budget_freq" => Ok(ValidCategoryFields::Freq),
            _ => Err(()),
        }
    }
}

pub async fn handle_category_update(
    email_str: String,
    field: String,
    category_nickname: String,
    new_value: String,
    pool: DbPool,
) -> (Status, String) {
    // Step 1: Check if the old entry exists in category table
    // Step 1.1: Check if a valid field is specified
    let field_str = field.as_str();
    let field_type = match ValidCategoryFields::from_str(field_str) {
        Ok(field_type) => field_type,
        Err(_) => return (Status::BadRequest, "Invalid field specified.".to_string()),
    };

    // Step 1.2: Check if user exists
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users
                .filter(user_email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    })
    .await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (
                Status::BadRequest,
                "No user found for the provided email".to_string(),
            );
        }
        Ok(Ok(Some(_user))) => {
            // User found, proceed to category name existence check
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (
                Status::InternalServerError,
                "Internal server error".to_string(),
            );
        }
    }

    // Step 1.3: Check if the category_nickname with old value entry exists in category table
    let nickname_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
        let name_to_check = category_nickname.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            categories
                .filter(email.eq(email_to_check))
                .filter(nickname.eq(name_to_check))
                .first::<Category>(&mut conn)
                .optional()
        }
    })
    .await;

    match nickname_exists {
        Ok(Ok(None)) => {
            // Category nickname does not exist, cannot update
            return (
                Status::BadRequest,
                "Failed to update category: unable to match existing entry".to_string(),
            );
        }
        Ok(Ok(Some(_))) => {
            // Step 2: Proceed to update the matching category
            let cat_to_change = category_nickname.clone();
            let email_to_change = email_str.clone();
            let value_to_change = new_value.clone();

            match field_type {
                ValidCategoryFields::Name => {
                    let result = tokio::task::spawn_blocking({
                        let pool = pool.clone();
                        move || {
                            let mut conn = pool.get().expect("Failed to get database connection");
                            diesel::update(
                                categories
                                    .filter(nickname.eq(cat_to_change))
                                    .filter(email.eq(email_to_change)),
                            )
                            .set(nickname.eq(value_to_change))
                            .execute(&mut conn)
                        }
                    })
                    .await;

                    match result {
                        Ok(Ok(_)) => {
                            // Successfully updated the category field
                            let msg = format!(
                                "Successfully updated category {} field {} to {}",
                                category_nickname,
                                field,
                                new_value.clone()
                            );
                            (Status::Created, msg)
                        }
                        Ok(Err(e)) => {
                            eprintln!("Database error during insertion: {:?}", e);
                            (Status::InternalServerError, "Database error".to_string())
                        }
                        Err(e) => {
                            eprintln!("Blocking task failed during insertion: {:?}", e);
                            (
                                Status::InternalServerError,
                                "Internal server error".to_string(),
                            )
                        }
                    }
                }
                ValidCategoryFields::Type => {
                    let result = tokio::task::spawn_blocking({
                        let pool = pool.clone();
                        move || {
                            let mut conn = pool.get().expect("Failed to get database connection");
                            diesel::update(
                                categories
                                    .filter(nickname.eq(cat_to_change))
                                    .filter(email.eq(email_to_change)),
                            )
                            .set(category_type.eq(value_to_change))
                            .execute(&mut conn)
                        }
                    })
                    .await;

                    match result {
                        Ok(Ok(_)) => {
                            // Successfully updated the category field
                            let msg = format!(
                                "Successfully updated category {} field {} to {}",
                                category_nickname,
                                field,
                                new_value.clone()
                            );
                            (Status::Created, msg)
                        }
                        Ok(Err(e)) => {
                            eprintln!("Database error during insertion: {:?}", e);
                            (Status::InternalServerError, "Database error".to_string())
                        }
                        Err(e) => {
                            eprintln!("Blocking task failed during insertion: {:?}", e);
                            (
                                Status::InternalServerError,
                                "Internal server error".to_string(),
                            )
                        }
                    }
                }
                ValidCategoryFields::Budget => {
                    let float_budget: f64 = value_to_change.parse().unwrap();
                    let result = tokio::task::spawn_blocking({
                        let pool = pool.clone();
                        move || {
                            let mut conn = pool.get().expect("Failed to get database connection");
                            diesel::update(
                                categories
                                    .filter(nickname.eq(cat_to_change))
                                    .filter(email.eq(email_to_change)),
                            )
                            .set(budget.eq(float_budget))
                            .execute(&mut conn)
                        }
                    })
                    .await;

                    match result {
                        Ok(Ok(_)) => {
                            // Successfully updated the category field
                            let msg = format!(
                                "Successfully updated category {} field {} to {}",
                                category_nickname,
                                field,
                                new_value.clone()
                            );
                            (Status::Created, msg)
                        }
                        Ok(Err(e)) => {
                            eprintln!("Database error during insertion: {:?}", e);
                            (Status::InternalServerError, "Database error".to_string())
                        }
                        Err(e) => {
                            eprintln!("Blocking task failed during insertion: {:?}", e);
                            (
                                Status::InternalServerError,
                                "Internal server error".to_string(),
                            )
                        }
                    }
                }
                ValidCategoryFields::Freq => {
                    let result = tokio::task::spawn_blocking({
                        let pool = pool.clone();
                        move || {
                            let mut conn = pool.get().expect("Failed to get database connection");
                            diesel::update(
                                categories
                                    .filter(nickname.eq(cat_to_change))
                                    .filter(email.eq(email_to_change)),
                            )
                            .set(budget_freq.eq(value_to_change))
                            .execute(&mut conn)
                        }
                    })
                    .await;
                    match result {
                        Ok(Ok(_)) => {
                            // Successfully updated the category field
                            let msg = format!(
                                "Successfully updated category {} field {} to {}",
                                category_nickname,
                                field,
                                new_value.clone()
                            );
                            (Status::Created, msg)
                        }
                        Ok(Err(e)) => {
                            eprintln!("Database error during insertion: {:?}", e);
                            (Status::InternalServerError, "Database error".to_string())
                        }
                        Err(e) => {
                            eprintln!("Blocking task failed during insertion: {:?}", e);
                            (
                                Status::InternalServerError,
                                "Internal server error".to_string(),
                            )
                        }
                    }
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error checking category existence: {:?}", e);
            (Status::InternalServerError, "Database error".to_string())
        }
        Err(e) => {
            eprintln!("Blocking task failed during category check: {:?}", e);
            (
                Status::InternalServerError,
                "Internal server error".to_string(),
            )
        }
    }
}
