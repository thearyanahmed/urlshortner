use sqlx::{Error, Pool, Postgres};
use sqlx::postgres::PgPoolOptions;
use crate::url_shortener::DataStore;
use async_trait::async_trait;

pub struct PostgresStore {
    con: Pool<Postgres>,
}

impl PostgresStore {
    pub fn new(connection_url: &str) -> Result<Self, Error> {
        // let con = PgPoolOptions::new()
        //     .connect(connection_url).await?;

        let con = PgPoolOptions::new()
            .connect_lazy(connection_url)?;

        Ok(Self {
            con,
        })
    }
}

#[async_trait]
impl DataStore for PostgresStore {
    async fn find_by_key(&self, _key: &str) -> Result<String, String> {
        println!("postgres find by key");


        Ok("some".to_string())
    }

    fn store(&self, _key: &str) -> Result<String, String> {
        Ok("some".to_string())
    }

    fn is_alive(&self) -> bool {
        !self.con.is_closed()
    }
}