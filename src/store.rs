use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::types::restaurant::{Restaurant, RestaurantId};

#[derive(Clone, Debug)]
pub struct Store {
    pub restaurants: Arc<RwLock<HashMap<RestaurantId, Restaurant>>>,
}

impl Store {
    pub fn init() -> Self {
        let mock_data = include_str!("../mock_data.json");
        match serde_json::from_str(mock_data) {
            Ok(data) => Store {
                restaurants: Arc::new(RwLock::new(data)),
            },
            Err(e) => {
                println!("there was an error reading mock_data.json: {e}");
                println!("initialized Empty...");
                Store::new()
            }
        }
    }
    pub fn new() -> Self {
        Store {
            restaurants: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
