use crate::api::types::{login::IdentificationMethod, register::RegisterReq};
use crate::db::{
    types::{user::register::RegisterError, User},
    DatabaseProvider,
};

#[derive(Debug, Clone)]
pub struct Users {
    provider: DatabaseProvider,
}

impl Users {
    pub fn new(provider: DatabaseProvider) -> Self {
        Self { provider }
    }

    pub async fn get_using_identifier(&self, identifier: IdentificationMethod) -> Option<User> {
        match identifier {
            IdentificationMethod::Email(email) => match &self.provider {
                DatabaseProvider::Sqlite(db) => sqlite_api::get_by_email(&db.pool, email).await,
                DatabaseProvider::Postgres(db) => postgres_api::get_by_email(&db.pool, email).await,
            },
            IdentificationMethod::Username(username) => match &self.provider {
                DatabaseProvider::Sqlite(db) => {
                    sqlite_api::get_by_username(&db.pool, username).await
                }
                DatabaseProvider::Postgres(db) => {
                    postgres_api::get_by_username(&db.pool, username).await
                }
            },
        }
    }

    pub async fn register(&self, user: RegisterReq) -> Result<(), RegisterError> {
        match &self.provider {
            DatabaseProvider::Sqlite(db) => sqlite_api::register(&db.pool, user).await,
            _ => todo!(),
        }
    }
}

mod sqlite_api {
    use sqlx::{Error, SqlitePool};

    use crate::db::types::User;

    use super::{RegisterError, RegisterReq};

    pub async fn get_by_email(pool: &SqlitePool, email: String) -> Option<User> {
        const QUERY: &str = "
                SELECT 
                    *
                FROM 
                    users u
                WHERE 
                    u.email = ?
            ";

        let user: anyhow::Result<User, Error> =
            sqlx::query_as(QUERY).bind(email).fetch_one(pool).await;

        match user {
            Ok(v) => Some(v),
            Err(Error::RowNotFound) => None,
            Err(err) => {
                dbg!("[SQLITE] Error getting user by email", err);
                None
            }
        }
    }

    pub async fn get_by_username(pool: &SqlitePool, username: String) -> Option<User> {
        const QUERY: &str = "
                SELECT 
                    *
                FROM 
                    users u
                WHERE 
                    u.username = ?
            ";

        let user: anyhow::Result<User, Error> =
            sqlx::query_as(QUERY).bind(username).fetch_one(pool).await;

        match user {
            Ok(v) => Some(v),
            Err(Error::RowNotFound) => None,
            Err(err) => {
                dbg!("[SQLITE] Error getting user by username", err);
                None
            }
        }
    }

    pub async fn register(
        pool: &SqlitePool,
        user: RegisterReq,
    ) -> anyhow::Result<(), RegisterError> {
        let RegisterReq {
            username,
            email,
            password,
        } = user;

        let user_id = uuid::Uuid::new_v4().to_string();
        let public_id = uuid::Uuid::new_v4().to_string();

        const QUERY: &str = "
                INSERT INTO users (user_id, public_id, username, email, password)
                VALUES (?, ?, ?, ?, ?)
            ";

        match sqlx::query(QUERY)
            .bind(user_id)
            .bind(public_id)
            .bind(username)
            .bind(email)
            .bind(password)
            .execute(pool)
            .await
        {
            Ok(v) => {
                dbg!(v);
                Ok(())
            }
            Err(Error::Database(err)) if err.is_unique_violation() => match err.message() {
                "UNIQUE constraint failed: users.email" => Err(RegisterError::EmailTaken),
                "UNIQUE constraint failed: users.username" => Err(RegisterError::UsernameTaken),
                _ => {
                    dbg!(err);
                    Err(RegisterError::Other)
                }
            },
            Err(err) => {
                dbg!(err);
                Err(RegisterError::Other)
            }
        }
    }
}

mod postgres_api {
    use sqlx::{Error, PgPool};

    use crate::db::types::User;

    pub async fn get_by_email(pool: &PgPool, email: String) -> Option<User> {
        const QUERY: &str = "
                SELECT 
                    *
                FROM 
                    users u
                WHERE 
                    u.username = $1
            ";

        let user: anyhow::Result<User, Error> =
            sqlx::query_as(QUERY).bind(email).fetch_one(pool).await;

        match user {
            Ok(v) => Some(v),
            Err(Error::RowNotFound) => None,
            Err(err) => {
                dbg!("[POSTGRES] Error getting user by email", err);
                None
            }
        }
    }

    pub async fn get_by_username(pool: &PgPool, username: String) -> Option<User> {
        const QUERY: &str = "
                SELECT 
                    *
                FROM 
                    users u
                WHERE 
                    u.username = $1
            ";

        let user: anyhow::Result<User, Error> =
            sqlx::query_as(QUERY).bind(username).fetch_one(pool).await;

        match user {
            Ok(v) => Some(v),
            Err(Error::RowNotFound) => None,
            Err(err) => {
                dbg!("[POSTGRES] Error getting user by username", err);
                None
            }
        }
    }
}
