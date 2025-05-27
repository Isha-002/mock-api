use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::orders::OrderStatus;



#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
  pub id: i32,
  pub account_id: Uuid,
  pub restaurant_id: i32,
  pub order_id: i32,
  pub order_status: OrderStatus,
  pub cash: bool,
  pub paid_money: Option<i32>,
  pub transaction_id: Option<String>,
  pub card_number: Option<String>,
}