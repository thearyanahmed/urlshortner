use crate::url_shortener::DataStore;

pub struct Postgres {

}

impl DataStore for Postgres {
    fn find_by_key(&self, key: &str) -> Result<String,String> {
        Ok("some".to_string())
    }

    fn store(&self, key: &str) -> Result<String,String> {
        Ok("some".to_string())
    }
}