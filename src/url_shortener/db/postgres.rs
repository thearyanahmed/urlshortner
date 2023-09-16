use crate::url_shortener::{DataStore, Url};
use async_trait::async_trait;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Error, Pool, Postgres};

pub struct PostgresStore {
    con: Pool<Postgres>,
}

impl PostgresStore {
    pub fn new(connection_url: &str) -> Result<Self, Error> {
        let con = PgPoolOptions::new().connect_lazy(connection_url)?;

        Ok(Self { con })
    }
}

#[async_trait]
impl DataStore for PostgresStore {
    async fn find_by_url(&mut self, original_url: &str) -> Result<Vec<Url>, Error> {
        sqlx::query_as::<_, Url>("SELECT * FROM urls WHERE original_url = $1 LIMIT 1")
            .bind(original_url)
            .fetch_all(&self.con)
            .await
    }

    fn store(&self, long_url: &str, short_url: &str) -> Result<String, String> {
        Ok("some".to_string())
    }

    fn is_alive(&self) -> bool {
        !self.con.is_closed()
    }
}
