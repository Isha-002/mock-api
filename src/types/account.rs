use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
#[allow(non_camel_case_types)]
pub enum Role {
  customer,
  restaurant_owner,
  banned_user,
  admin,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Account {
  pub id: i32,
  pub email: String,
  pub password: String,
  pub phone_number: String,
  pub role: Role
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewAccount {
  pub email: String,
  pub password: String,
  pub phone_number: String
}