#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub key: String,
}

#[derive(serde::Serialize)]
pub struct UrlCacheRecord {
    pub original_url: String,
    pub key: String,
}

#[derive(serde::Serialize)]
pub struct TinyUrlResponse {
    pub url: String,
}

impl Url {
    pub fn to_tiny_url_response(&self, base_url: String) -> TinyUrlResponse {
        let url = format!("{}/{}",base_url, self.key);

        TinyUrlResponse {
            url
        }
    }
}

