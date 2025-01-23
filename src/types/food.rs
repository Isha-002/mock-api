use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Food {
    pub name: String,
    pub price: usize,
    pub ingredient: Option<Vec<String>>,
    pub available: bool,
}
