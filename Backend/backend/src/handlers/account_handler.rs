use crate::models::account::{NewAccount, Account};
use crate::models::user::User;
use crate::schema::accounts::dsl::*;
use crate::schema::users::dsl::{users, email as user_email}; // For users table
use diesel::prelude::*;
use crate::db::DbPool;
use rocket::http::Status;
use rocket::serde::json::Json;

// DELETE delete account
pub async fn handle_delete_account(email_str: String, account_name_str: String, pool: DbPool) -> (Status, &'static str) {
    // Check if email is empty or account_name is empty
    if email_str.is_empty() || account_name_str.is_empty() {
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
    }).await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (Status::BadRequest, "No user found for the provided email");
        }
        Ok(Ok(Some(_))) => {
            // User found, proceed with account deletion
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

    // Check if the account exists for this user
    let account_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = email_str.clone();
        let acc_name_to_check = account_name_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            accounts
                .filter(email.eq(email_to_check))
                .filter(account_name.eq(acc_name_to_check))
                .first::<Account>(&mut conn)
                .optional()
        }
    }).await;

    let found_account = match account_exists {
        Ok(Ok(Some(acc))) => acc,
        Ok(Ok(None)) => {
            // Account not found for this email
            return (Status::BadRequest, "No such account found for the provided email");
        }
        Ok(Err(e)) => {
            eprintln!("Error checking account existence: {:?}", e);
            return (Status::InternalServerError, "Database error");
        }
        Err(e) => {
            eprintln!("Blocking task failed during account existence check: {:?}", e);
            return (Status::InternalServerError, "Internal server error");
        }
    };

    // Proceed to delete the found account
    let deletion_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let acc_id_to_delete = found_account.account_id;
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            diesel::delete(accounts.filter(account_id.eq(acc_id_to_delete)))
                .execute(&mut conn)
        }
    }).await;

    match deletion_result {
        Ok(Ok(rows_deleted)) => {
            if rows_deleted > 0 {
                (Status::Ok, "Account successfully deleted")
            } else {
                (Status::InternalServerError, "Failed to delete the account")
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error during deletion: {:?}", e);
            (Status::InternalServerError, "Database error during deletion")
        }
        Err(e) => {
            eprintln!("Blocking task failed during deletion: {:?}", e);
            (Status::InternalServerError, "Internal server error")
        }
    }
}

// GET /account_summary?email=<>
pub async fn handle_account_summary(email_str: String, pool: DbPool) -> (Status, Json<Vec<Account>>) {
    // If email is empty, return bad request
    if email_str.is_empty() {
        return (Status::BadRequest, Json(vec![]));
    }

    let accounts_result = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_search = email_str.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            accounts
                .filter(email.eq(email_to_search))
                .load::<Account>(&mut conn)
        }
    }).await;

    match accounts_result {
        Ok(Ok(acc_list)) => {
            // Successfully retrieved accounts
            (Status::Ok, Json(acc_list))
        }
        Ok(Err(e)) => {
            eprintln!("Database error during account summary retrieval: {:?}", e);
            (Status::InternalServerError, Json(vec![]))
        }
        Err(e) => {
            eprintln!("Blocking task failed during account summary retrieval: {:?}", e);
            (Status::InternalServerError, Json(vec![]))
        }
    }
}

pub async fn handle_account_create(new_acc: NewAccount, pool: DbPool) -> (Status, String) {
    // Step 1: Validate input
    if new_acc.email.is_empty() || new_acc.account_type.is_empty() || new_acc.account_name.is_empty() {
        return (Status::BadRequest, "Invalid input".to_string());
    }

    // Step 1.5: Check if the email exists in users table
    // If not, no account should be created
    let user_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_acc.email.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users
                .filter(user_email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    }).await;

    match user_exists {
        Ok(Ok(None)) => {
            // No user found for this email
            return (Status::BadRequest, "No user found for the provided email".to_string());
        }
        Ok(Ok(Some(_user))) => {
            // User found, proceed to account name existence check
        }
        Ok(Err(e)) => {
            eprintln!("Error checking user existence: {:?}", e);
            return (Status::InternalServerError, "Database error".to_string());
        }
        Err(e) => {
            eprintln!("Blocking task failed during user check: {:?}", e);
            return (Status::InternalServerError, "Internal server error".to_string());
        }
    }

    // Step 2: Check if the account_name already exists for the given email
    let account_name_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = new_acc.email.clone();
        let name_to_check = new_acc.account_name.clone();
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            accounts
                .filter(email.eq(email_to_check))
                .filter(account_name.eq(name_to_check))
                .first::<Account>(&mut conn)
                .optional()
        }
    }).await;

    match account_name_exists {
        Ok(Ok(Some(_existing_acc))) => {
            // Account name already taken for this email
            return (Status::BadRequest, "Failed to create new account".to_string());
        }
        Ok(Ok(None)) => {
            // Step 3: Proceed to create the new account
            let acc_to_insert = new_acc.clone(); // Clone the NewAccount to avoid moving it
            let acc_for_message = new_acc.clone(); // Another clone to access account_name later
            let result = tokio::task::spawn_blocking({
                let pool = pool.clone();
                move || {
                    let mut conn = pool.get().expect("Failed to get database connection");
                    diesel::insert_into(accounts)
                        .values(&acc_to_insert) // Use one copy of the NewAccount
                        .execute(&mut conn)
                }
            }).await;

            match result {
                Ok(Ok(_)) => {
                    // Successfully inserted the account
                    let msg = format!("Successfully created {}", acc_for_message.account_name);
                    (Status::Created, msg)
                }
                Ok(Err(e)) => {
                    eprintln!("Database error during insertion: {:?}", e);
                    (Status::InternalServerError, "Database error".to_string())
                }
                Err(e) => {
                    eprintln!("Blocking task failed during insertion: {:?}", e);
                    (Status::InternalServerError, "Internal server error".to_string())
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error checking account existence: {:?}", e);
            (Status::InternalServerError, "Database error".to_string())
        }
        Err(e) => {
            eprintln!("Blocking task failed during account check: {:?}", e);
            (Status::InternalServerError, "Internal server error".to_string())
        }
    }
}
