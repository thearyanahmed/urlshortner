#[derive(sqlx::FromRow, Debug, serde::Serialize)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_url: String,
}
