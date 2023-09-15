use crate::url_shortener::DataStore;

pub struct PostgresStore {

}

impl PostgresStore {
    pub fn new() -> Self {
        Self {}
    }
}

impl DataStore for PostgresStore {
    fn find_by_key(&self, key: &str) -> Result<String,String> {
        println!("postgres find by key");
        Ok("some".to_string())
    }

    fn store(&self, key: &str) -> Result<String,String> {
        Ok("some".to_string())
    }
}