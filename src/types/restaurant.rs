use std::fmt;

use serde::{Deserialize, Serialize};

use super::food::Food;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Restaurant {
    pub id: RestaurantId,
    pub name: String,
    pub rating: f32,
    pub distance: f64, // my frontend guy is too lazy, so...
    pub tags: Option<Vec<String>>,
    pub menu: Vec<Food>,
    pub image: String,
    pub city: String,
    pub address: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct RestaurantId(pub i32);

impl fmt::Display for Restaurant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{{ id: {}, name: {}, rating: {}, distance: {}, tags: {:?}, menu: {:?} image: {} }}`",
            self.id, self.name, self.rating, self.distance, self.tags, self.menu, self.image
        )
    }
}

impl fmt::Display for RestaurantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewRestaurant {
    pub name: String,
    pub rating: f32,
    pub distance: f64,
    pub tags: Option<Vec<String>>,
    pub menu: Vec<Food>,
    pub image: String,
    pub city: String,
    pub address: String,
}
