use crate::db::{types::BookedDay, DatabaseProvider};

#[derive(Debug, Clone)]
pub struct BookedDays {
    provider: DatabaseProvider,
}

impl BookedDays {
    pub fn new(provider: DatabaseProvider) -> Self {
        Self { provider }
    }

    pub async fn get_during(&self, year: u16, month: u8) -> anyhow::Result<Vec<BookedDay>> {
        match &self.provider {
            DatabaseProvider::Postgres(db) => postgres_api::get_during(&db.pool, year, month).await,
            DatabaseProvider::Sqlite(db) => sqlite_api::get_during(&db.pool, year, month).await,
        }
    }
}

mod sqlite_api {
    use sqlx::SqlitePool;

    use crate::db::types::BookedDay;

    pub async fn get_during(
        pool: &SqlitePool,
        year: u16,
        month: u8,
    ) -> anyhow::Result<Vec<BookedDay>> {
        const QUERY: &str = "
                SELECT 
                    CAST(strftime('%d', bd.date) AS INTEGER) AS day,
                    bd.booked_by
                FROM 
                    booked_days bd
                WHERE 
                    strftime('%Y', bd.date) = ?
                    AND strftime('%m', bd.date) = ?
            ";

        Ok(sqlx::query_as(QUERY)
            .bind(year)
            // postgres thinks an i8 is a char
            .bind(month as u16)
            .fetch_all(pool)
            .await?)
    }
}

mod postgres_api {
    use sqlx::PgPool;

    use crate::db::types::BookedDay;

    pub async fn get_during(pool: &PgPool, year: u16, month: u8) -> anyhow::Result<Vec<BookedDay>> {
        const QUERY: &str = "
                SELECT
                  CAST(EXTRACT(DAY FROM bd.date) AS INT) AS day,
                  bd.booked_by
                FROM
                  booked_days bd
                WHERE
                  EXTRACT(YEAR FROM bd.date) = $1
                  AND EXTRACT(MONTH FROM bd.date) = $2
            ";

        Ok(sqlx::query_as(QUERY)
            .bind(year as i16)
            // postgres thinks an i8 is a char
            .bind(month as i16)
            .fetch_all(pool)
            .await?)
    }
}
