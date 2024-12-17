use crate::models::user::{NewUser, User};
use crate::schema::users::dsl::*;
use diesel::prelude::*;
#[allow(unused_imports)]
use diesel::result::Error;
use crate::db::DbPool;
use rocket::http::Status;

pub async fn handle_signup(user: NewUser, pool: DbPool) -> (Status, &'static str) {
    if user.email.is_empty() || user.password.is_empty() {
        return (Status::BadRequest, "Invalid input");
    }

    println!("Signup request received for user: {}", user.username);
    println!("Signup request received for email: {}", user.email);

    // Check if the email is already registered
    let email_exists = tokio::task::spawn_blocking({
        let pool = pool.clone();
        let email_to_check = user.email.clone(); // Clone email to avoid move
        move || {
            let mut conn = pool.get().expect("Failed to get database connection");
            users.filter(email.eq(email_to_check))
                .first::<User>(&mut conn)
                .optional()
        }
    })
        .await;

    match email_exists {
        Ok(Ok(Some(existing_user))) => {
            if existing_user.password == user.password {
                println!("Email and password match for user: {}", existing_user.username);
                let message = format!("{} Login successful", existing_user.username); // Include username in the response
                return (Status::Ok, Box::leak(message.into_boxed_str()));
            } else {
                println!("Password mismatch for email: {}", existing_user.email);
                return (Status::BadRequest, "Invalid password");
            }
        }
        Ok(Ok(None)) => {
            // Email does not exist, create the new user
            let result = tokio::task::spawn_blocking({
                let new_user = user.clone(); // Clone user to avoid move
                let pool = pool.clone();
                move || {
                    let mut conn = pool.get().expect("Failed to get database connection");
                    diesel::insert_into(users)
                        .values(&new_user)
                        .execute(&mut conn)
                }
            })
                .await;

            match result {
                Ok(Ok(_)) => {
                    println!("User successfully registered: {}", user.username); // We can now use user here
                    (Status::Created, "User successfully registered")
                }
                Ok(Err(diesel::result::Error::DatabaseError(
                           diesel::result::DatabaseErrorKind::UniqueViolation,
                           _,
                       ))) => {
                    eprintln!("Duplicate email insertion error.");
                    (Status::Conflict, "Email already registered")
                }
                Ok(Err(e)) => {
                    eprintln!("Database error during insertion: {:?}", e);
                    (Status::InternalServerError, "Failed to register user")
                }
                Err(e) => {
                    eprintln!("Blocking task failed during insertion: {:?}", e);
                    (Status::InternalServerError, "Internal server error")
                }
            }
        }
        Ok(Err(e)) => {
            eprintln!("Error checking email existence: {:?}", e);
            (Status::InternalServerError, "Database error")
        }
        Err(e) => {
            eprintln!("Blocking task failed during email check: {:?}", e);
            (Status::InternalServerError, "Internal server error")
        }
    }
}
