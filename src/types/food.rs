use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct Food {
    pub id: i32,
    pub restaurant_id: i32,
    pub name: String,
    pub image: String,
    pub tag: String,
    pub price: i32,
    pub discount: bool,
    pub discount_price: Option<i32>,
    pub ingredient: Vec<String>,
    pub available: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
pub struct NewFood {
    pub restaurant_id: i32,
    pub name: String,
    pub image: String,
    pub tag: String,
    pub price: i32,
    pub discount: bool,
    pub discount_price: Option<i32>,
    pub ingredient: Vec<String>,
    pub available: bool,
}
