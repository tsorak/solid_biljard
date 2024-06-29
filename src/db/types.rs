use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BookedDay {
    day: i8,
    booked_by: String,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct User {
    user_id: String,
    public_id: String,
    email: String,
    username: String,
    password: String,
}

pub mod user {
    use argon2::{Argon2, PasswordHash, PasswordVerifier};

    use super::User;

    impl User {
        pub fn is_correct_password(&self, password: String) -> bool {
            let actual_password =
                PasswordHash::new(&self.password).expect("Failed to parse password from db");

            Argon2::default()
                .verify_password(password.as_bytes(), &actual_password)
                .is_ok()
        }
    }

    pub mod register {
        pub enum RegisterError {
            EmailTaken,
            UsernameTaken,
            Other,
        }
    }
}
