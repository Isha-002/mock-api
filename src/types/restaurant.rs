use std::fmt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Restaurant {
    pub id: RestaurantId,
    pub name: String,
    pub rating: f32,
    pub distance: f64,
    pub tags: Option<Vec<String>>,
    pub image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct RestaurantId(pub String);


impl fmt::Display for Restaurant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{{ id: {}, name: {}, rating: {}, distance: {}, tags: {:?}, image: {} }}`",
            self.id, self.name, self.rating, self.distance, self.tags, self.image
        )
    }
}

impl fmt::Display for RestaurantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}
