#[derive(sqlx::FromRow, Debug, serde::Serialize, Clone)]
pub struct Visit {
    pub id: i32,
    pub key: String,
}