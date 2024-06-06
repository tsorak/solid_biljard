use serde::{Deserialize, Serialize};
use sqlx::{prelude::FromRow, PgPool};

#[derive(Debug, Clone)]
pub struct BookedDays {
    pool: PgPool,
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct BookedDay {
    day: i8,
    booked_by: String,
}

impl BookedDays {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn get_during(&self, year: u16, month: u8) -> anyhow::Result<Vec<BookedDay>> {
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
            .fetch_all(&self.pool)
            .await?)
    }
}
