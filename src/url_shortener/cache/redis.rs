use crate::url_shortener::CacheStore;
use async_trait::async_trait;

pub struct RedisStore {
    con: redis::Connection,
}

impl RedisStore {
    pub fn new(connection_url: &str) -> Self {
        let client = redis::Client::open(connection_url)
            .expect("could not connect to redis");

        let con = client
            .get_connection()
            .expect("could not get connection to redis");

        Self { con }
    }
}

#[async_trait]
impl CacheStore for RedisStore {
    fn is_alive(&mut self) -> bool {
        match redis::cmd("PING").query::<String>(&mut self.con) {
            Ok(res) => res == "PONG",
            Err(_) => false,
        }
    }


    fn find_by_key(&mut self, key: &str) -> Result<String, String> {
        match redis::cmd("GET").arg("hello").query::<String>(&mut self.con) {
            Ok(_url) => {
                Ok("OK called".to_string())
            }
            Err(_) => Err("Error called".to_string()),
        }
    }

    async fn store(&mut self, key: &str, value: &str) -> Result<bool, String> {
        match redis::cmd("SET").arg(key).arg(value).query::<String>(&mut self.con) {
            Ok(_) => Ok(true),
            Err(err) => Err(err.to_string()),
        }
    }
}