use serde::{Deserialize, Serialize};
use uuid::Uuid;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Owner {
    pub restaurant_id: i32,
    pub account_id: Uuid,
    pub national_id: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewOwner {
    pub restaurant_id: i32,
    pub national_id: String, 
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOwner {
    pub name: String,
    pub phone_number: String,
    pub email: String,
    pub national_id: String, 
}