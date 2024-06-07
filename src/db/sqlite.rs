use sqlx::sqlite::SqlitePool;

const CONNECT_URL: &str = "sqlite://dev.db";

#[derive(Debug, Clone)]
pub struct DB {
    pub pool: SqlitePool,
}

impl DB {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Self {
        println!("Connecting to Sqlite database...");

        let pool = SqlitePool::connect(CONNECT_URL)
            .await
            .expect("Failed to connect to database");

        println!("Connected to Sqlite database");

        Self::new(pool)
    }
}
