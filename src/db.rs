mod postgres;
mod sqlite;

mod accessors;
pub mod types;

#[derive(Debug, Clone)]
pub enum DatabaseProvider {
    Postgres(postgres::DB),
    Sqlite(sqlite::DB),
}

#[derive(Debug, Clone)]
pub struct DB {
    // provider: DatabaseProvider,
    pub booked_days: accessors::booked_days::BookedDays,
    pub users: accessors::users::Users,
}

impl DB {
    fn new(provider: DatabaseProvider) -> Self {
        Self {
            booked_days: accessors::booked_days::BookedDays::new(provider.clone()),
            users: accessors::users::Users::new(provider),
        }
    }

    pub async fn new_postgres() -> Self {
        let pool = postgres::DB::connect().await;
        let provider = DatabaseProvider::Postgres(pool);

        Self::new(provider)
    }

    pub async fn new_sqlite() -> Self {
        let pool = sqlite::DB::connect().await;
        let provider = DatabaseProvider::Sqlite(pool);

        Self::new(provider)
    }
}
