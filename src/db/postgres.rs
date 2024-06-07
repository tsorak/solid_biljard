use sqlx::postgres::PgPool;

const CONNECT_URL: &str = "postgres://postgres:root@localhost/biljard";

#[derive(Debug, Clone)]
pub struct DB {
    pub pool: PgPool,
}

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn connect() -> Self {
        println!("Connecting to Postgres database...");

        let pool = PgPool::connect(CONNECT_URL)
            .await
            .expect("Failed to connect to database");

        println!("Connected to Postgres database");

        Self::new(pool)
    }
}
