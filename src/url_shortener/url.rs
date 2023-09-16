#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub key: String,
}

#[derive(serde::Serialize)]
pub struct TinyUrl {
    pub url: String,
}

impl Url {
    pub fn to_tiny_url(&self, base_url: String) -> TinyUrl {
        let url = format!("{}/{}",base_url, self.key);

        TinyUrl {
            url
        }
    }
}

