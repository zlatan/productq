use serde::{Serialize, Deserialize};
use chrono::NaiveDateTime;

#[derive(Debug, Serialize, Deserialize)]
pub struct Items {
    pub items: Vec<Product>
}

#[derive(sqlx::FromRow, Serialize, Debug, Deserialize)]
pub struct Product{
    pub title: String,
    pub price: f64,
    pub currency: String,
    pub image_url: String,
    pub origin_url: String,
    pub last_update: NaiveDateTime,
    pub tags: sqlx::types::Json<Vec<String>>,
}