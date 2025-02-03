use serde::{Deserialize, Serialize};
use sqlx::Type;

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[sqlx(type_name = "food")] 
pub struct Food {
    pub name: String,
    pub price: i32,
    pub ingredient: Option<Vec<String>>,
    pub available: bool,
}

