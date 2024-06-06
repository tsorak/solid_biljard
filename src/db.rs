mod booked_days;
mod users;

use sqlx::postgres::PgPool;

const CONNECT_URL: &str = "postgres://postgres:root@localhost/biljard";

#[derive(Debug, Clone)]
pub struct DB {
    // pool: PgPool,
    pub booked_days: booked_days::BookedDays,
    // users: users::Users,
}

impl DB {
    pub fn new(pool: PgPool) -> Self {
        Self {
            // pool: pool.clone(),
            booked_days: booked_days::BookedDays::new(pool.clone()),
            // users: users::Users::new(pool.clone()),
        }
    }

    pub async fn connect() -> Self {
        let pool = PgPool::connect(CONNECT_URL)
            .await
            .expect("Failed to connect to database");

        Self::new(pool)
    }

    // pub async fn connect_to(url: &str) -> Self {
    //     let pool = PgPool::connect(url)
    //         .await
    //         .expect("Failed to connect to database");
    //
    //     Self::new(pool)
    // }
}
