use crate::url_shortener::CacheStore;

pub struct RedisStore {
    con: redis::Connection,
}

impl RedisStore {
    pub fn new() -> Self {
        // @TODO take connection string as parameter.
        let client = redis::Client::open("redis://127.0.0.1/").expect("could not connect to redis");

        let con = client
            .get_connection()
            .expect("could not get connection to redis");

        Self { con }
    }
}

impl CacheStore for RedisStore {
    fn is_alive(&mut self) -> bool {
        match redis::cmd("PING").query::<String>(&mut self.con) {
            Ok(res) => res == "PONG",
            Err(_) => false,
        }
    }


    fn find_by_key(&mut self, key: &str) -> Result<String, String> {
        println!("calling from redis store, got key {}",key);

        match redis::cmd("GET").arg("hello").query::<String>(&mut self.con) {
            Ok(_url) => {
                Ok("OK called".to_string())
            }
            Err(_) => Err("Error called".to_string()),
        }
    }

    fn store(&self, key: &str, value: &str) -> Result<String, String> {
        Ok("world".to_string())
    }
}