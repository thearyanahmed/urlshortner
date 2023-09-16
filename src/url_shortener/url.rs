#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct Url {
    pub id: i32,
    pub original_url: String,
    pub short_url: String,
}

impl Url {
    // presenter? @todo
}