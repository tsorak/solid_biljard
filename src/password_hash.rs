use std::{path::PathBuf, str::FromStr, sync::Arc};

use argon2::{
    password_hash::{rand_core::OsRng, Salt, SaltString},
    Argon2, PasswordHasher as _,
};

#[derive(Debug, Clone)]
pub struct PasswordHasher {
    salt: Arc<String>,
}

impl PasswordHasher {
    pub async fn init(path: &str) -> Self {
        let path = PathBuf::from_str(path).expect("Invalid path specified");
        let salt = match path.is_file() {
            true => Self::get_salt_from_file(path).await,
            false => Self::new_salt(path).await,
        };

        Self {
            salt: Arc::new(salt),
        }
    }

    async fn new_salt(_save_path: PathBuf) -> String {
        let salt = SaltString::generate(&mut OsRng);

        // maybe save salt to disk?
        // let mut salt_data = [0; 1024];
        // let _ = salt.decode_b64(salt_data.as_mut());
        // let _ = tokio::fs::write(&save_path, salt_data).await;

        salt.as_str().to_string()
    }

    async fn get_salt_from_file(path: PathBuf) -> String {
        let s = tokio::fs::read_to_string(path)
            .await
            .expect("Failed to read salt file");

        let salt = SaltString::from_b64(&s).expect("Failed to parse salt file");

        salt.to_string()
    }

    pub fn hash(&self, password: String) -> String {
        let hashed = Argon2::default()
            .hash_password(password.as_bytes(), Salt::from_b64(&self.salt).unwrap())
            .unwrap();

        hashed.serialize().as_str().to_string()
    }
}
