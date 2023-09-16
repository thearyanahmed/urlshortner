use crate::url_shortener::{DataStore, Url, Visit};
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
    async fn find_by_url(&mut self, original_url: &str, limit: u8) -> Result<Vec<Url>, Error> {
        self.find_by("original_url", original_url,limit).await
    }

    async fn find_by_key(&mut self, key: &str, limit: u8) -> Result<Vec<Url>, Error> {
        self.find_by("key", key,limit).await
    }

    async fn store_url(&self, original_url: &str, key: &str) -> Result<Url, Error> {
        sqlx::query_as::<_, Url>(r#"INSERT INTO urls ( original_url, key ) VALUES ( $1, $2 ) returning id, original_url, key"#)
            .bind(original_url)
            .bind(key)
            .fetch_one(&self.con)
            .await
    }

    async fn store_visit(&self, key: &str) -> Result<Visit, Error> {
        sqlx::query_as::<_, Visit>(r#"INSERT INTO visits ( key ) VALUES ( $1 ) returning id, key"#)
            .bind(key)
            .fetch_one(&self.con)
            .await
    }

    fn is_alive(&self) -> bool {
        !self.con.is_closed()
    }
}

impl PostgresStore {
    async fn find_by(&mut self, column: &str, value: &str, limit: u8) -> Result<Vec<Url>, Error> {
        let query = format!("SELECT * FROM urls WHERE {} = $1 LIMIT {}",column, limit);

        sqlx::query_as::<_, Url>(&*query)
            .bind(value)
            .fetch_all(&self.con)
            .await
    }
}