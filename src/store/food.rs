use crate::{
    error::Error,
    types::{account::Role, food::{Food, NewFood}}, verify_roles,
};
use sqlx::Row;
use uuid::Uuid;

use super::Store;

impl Store {
    pub async fn get_menu(&self, restaurant_id: i32) -> Result<Vec<Food>, Error> {
        match sqlx::query(
            "SELECT * from Food
          WHERE restaurant_id = $1",
        )
        .bind(restaurant_id)
        .map(|row| Food {
            id: row.get("id"),
            restaurant_id: row.get("restaurant_id"),
            name: row.get("name"),
            image: row.get("image"),
            tag: row.get("tag"),
            price: row.get("price"),
            discount: row.get("discount"),
            discount_price: row.get("discount_price"),
            ingredient: row.get("ingredient"),
            available: row.get("available"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(foods) => Ok(foods),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn post_new_food(&self, food: NewFood, account_id: Uuid) -> Result<Food, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
          Ok(true) => {
            match sqlx::query(
              "INSERT INTO food (restaurant_id, name, image, tag, price, discount, discount_price, ingredient, available)
              VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
              RETURNING *;"
            )
            .bind(food.restaurant_id) 
            .bind(food.name) 
            .bind(food.image) 
            .bind(food.tag) 
            .bind(food.price) 
            .bind(food.discount) 
            .bind(food.discount_price) 
            .bind(food.ingredient) 
            .bind(food.available)
            .map(|row| Food {
              id: row.get("id"),
              restaurant_id: row.get("restaurant_id"),
              name: row.get("name"),
              image: row.get("image"),
              tag: row.get("tag"),
              price: row.get("price"),
              discount: row.get("discount"),
              discount_price: row.get("discount_price"),
              ingredient: row.get("ingredient"),
              available: row.get("available"),
          })
          .fetch_one(&self.connection)
          .await
            {
              Ok(food) => Ok(food),
              Err(e) => Err(Error::database_query_error(e)),
            }
          }
          Ok(false) => Err(Error::not_authorized),
          Err(e) => Err(e),
        }
    }

    pub async fn update_food(&self, food: Food, account_id: Uuid) -> Result<Food, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
          Ok(true) => {
            match sqlx::query(
              "UPDATE food
              SET name = $1, image = $2, tag = $3, price = $4, discount = $5, discount_price = $6, ingredient = $7, available = $8
              WHERE id = $9 AND restaurant_id = $10
              RETURNING *;"
            )
            .bind(food.name) 
            .bind(food.image) 
            .bind(food.tag) 
            .bind(food.price) 
            .bind(food.discount) 
            .bind(food.discount_price) 
            .bind(food.ingredient) 
            .bind(food.available)
            .bind(food.id)
            .bind(food.restaurant_id)
            .map(|row| Food {
              id: row.get("id"),
              restaurant_id: row.get("restaurant_id"),
              name: row.get("name"),
              image: row.get("image"),
              tag: row.get("tag"),
              price: row.get("price"),
              discount: row.get("discount"),
              discount_price: row.get("discount_price"),
              ingredient: row.get("ingredient"),
              available: row.get("available"),
          })
          .fetch_one(&self.connection)
          .await
            {
              Ok(food) => Ok(food),
              Err(e) => Err(Error::database_query_error(e)),
            }
          }
          Ok(false) => Err(Error::not_authorized),
          Err(e) => Err(e),
        }
    }

    pub async fn delete_food(&self, food: Food, account_id: Uuid) -> Result<bool, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
          Ok(true) => {
            match sqlx::query(
              "DELETE FROM food
              WHERE id = $1 AND restaurant_id = $2;"
            )
            .bind(food.id)
            .bind(food.restaurant_id)
            .execute(&self.connection)
            .await 
            {
              Ok(_) => Ok(true),
              Err(e) => Err(Error::database_query_error(e)),
            }
          }
          Ok(false) => Err(Error::not_authorized),
          Err(e) => Err(e),
        }
    }

    pub async fn insert_food_image(
      &self,
      url: &str,
      food_id: i32,
      restaurant_id: i32,
  ) -> Result<bool, Error> {
      match sqlx::query(
          "UPDATE food 
          SET image = $1
          WHERE id = $2 AND restaurant_id = $3",
      )
      .bind(url)
      .bind(food_id)
      .bind(restaurant_id)
      .execute(&self.connection)
      .await
      {
          Ok(_) => Ok(true),
          Err(e) => Err(Error::database_query_error(e)),
      }
  }
}
