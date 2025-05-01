use serde::{Deserialize, Serialize};
use sqlx::Type;


#[derive(
  Deserialize, 
  Serialize, 
  Debug, 
  Clone, 
  Type,      // Add this   // Add this
)]
#[sqlx(type_name = "role")]  // Must match PostgreSQL enum type
#[sqlx(rename_all = "lowercase")]  // Match enum variant casing
#[allow(non_camel_case_types)]
pub enum Role {
  customer,
  restaurant_owner,
  banned_user,
  admin,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
  pub id: String,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub role: Role
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAccount {
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub role: Role
}