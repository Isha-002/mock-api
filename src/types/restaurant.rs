use std::fmt;

use chrono::NaiveTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Restaurant {
    pub id: RestaurantId,
    pub name: String,
    pub rating: f32,
    pub distance: f64,
    pub tags: Option<Vec<String>>,
    pub image: String,
    pub address: String,
    pub city: String,
    pub location: [f64; 2],
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
pub struct RestaurantId(pub i32);

impl fmt::Display for Restaurant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{{ id: {}, name: {}, rating: {}, distance: {}, tags: {:?},  image: {}, address: {}, city: {}, location: {:?} }}`",
            self.id, self.name, self.rating, self.distance, self.tags, self.image, self.address, self.city, self.location
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
    pub image: String,
    pub address: String,
    pub city: String,
    pub location: [f64; 2],
}

#[derive(Serialize, Deserialize, Clone, Debug)]

pub struct OpenHours {
    pub restaurant_id: i32,
    pub day_of_week: Weekday,
    pub open_time: NaiveTime,
    pub close_time: NaiveTime,
}
#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[sqlx(type_name = "weekday")]
pub enum Weekday {
    Saturday,
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
}

impl Weekday {
    pub fn _to_str(&self) -> &'static str {
        match self {
            Self::Saturday => "شنبه",
            Self::Sunday => "یکشنبه",
            Self::Monday => "دوشنبه",
            Self::Tuesday => "سه‌ شنبه",
            Self::Wednesday => "چهارشنبه",
            Self::Thursday => "پنجشنبه",
            Self::Friday => "جمعه",
        }
    }

    pub fn _from_str(day: &str) -> Option<Self> {
        match day {
            "شنبه" => Some(Self::Saturday),
            "یکشنبه" | "یک شنبه" => Some(Self::Sunday),
            "دوشنبه" | "دو شنبه" => Some(Self::Monday),
            "سه‌ شنبه" | "سه‌شنبه" => Some(Self::Tuesday),
            "چهارشنبه" | "چهار شنبه" => Some(Self::Wednesday),
            "پنجشنبه" | "پنج شنبه" => Some(Self::Thursday),
            "جمعه" => Some(Self::Friday),
            _ => None,
        }
    }
}