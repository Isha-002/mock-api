use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;


#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Order {
  pub id: i32,
  pub account_id: Uuid,
  pub status: OrderStatus,
  pub total_price: i32,
  pub total_discounted_price: i32,
  pub created_at: NaiveDateTime,
}

// user adds a food to his cart:
// we make a cart and set the status to "cart"
// user paid? set the status to "pending"
// restaurant served the food? set the status to "completed" -> should be done by restaurant manually
// if user cancelled the order after payment it's up to restaurant to resolve the order. in that case restaurant must set the status to "canceled"
// we can use this data to ban a user if "canceled" orders reach a limit!
// frontend can send status as "completed" after a period if it wasnt done by restaurant

#[derive(Serialize, Deserialize, Clone, Debug, Type)]
#[sqlx(type_name = "text")]
pub enum OrderStatus {
    Cart,
    Pending,
    Completed,
    Canceled,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Item {
  pub id: i32,
  pub order_id: i32,
  pub restaurant_id: i32,
  pub food_id: i32,
  pub quantity: i32,
  pub account_id: Uuid,
  pub name: String,
  pub image: String,
  pub price: i32,
  pub discount_price: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NewItem {
  pub order_id: i32,
  pub restaurant_id: i32,
  pub food_id: i32,
  pub quantity: i32,
  pub name: String,
  pub image: String,
  pub price: i32,
  pub discount_price: Option<i32>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct GetOrder {
  pub items: Vec<Item>,
  pub account_id: Uuid,
  pub order_id: i32,
  pub status: OrderStatus,
  pub total_price: i32,
  pub total_discounted_price: i32,
}


impl OrderStatus {
  pub fn _to_str(&self) -> &'static str {
    match self {
        Self::Cart => "cart",
        Self::Pending => "pending",
        Self::Completed => "completed",
        Self::Canceled => "canceled",
    }
}

  pub fn _from_str(s: &str) -> Option<Self> {
    match s.to_lowercase().as_str() {
        "cart" => Some(OrderStatus::Cart),
        "pending" => Some(OrderStatus::Pending),
        "completed" => Some(OrderStatus::Completed),
        "canceled" => Some(OrderStatus::Canceled),
        _ => None,  
    }
}
}