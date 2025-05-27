use uuid::Uuid;
use sqlx::{postgres::PgRow, Row};
use crate::{error::Error, types::orders::{GetOrder, Item, NewItem}};

use super::Store;


impl Store {
    pub async fn get_customer_orders(&self, account_id: Uuid, order_id: i32) -> Result<GetOrder, Error> {
      // i guess this query can be optimized but for now i just wanna make it work
      // TODO
        let order: PgRow = match sqlx::query(
          "SELECT
            orders.id AS order_id,
            orders.status,
            COALESCE(SUM(item.price * item.quantity), 0) AS total_price,
            SUM(item.discount_price * item.quantity) AS total_discounted_price
          FROM orders 
          LEFT JOIN items ON orders.id = item.order_id
          WHERE orders.id = $1 OR (orders.account_id = $2 AND orders.status IN ('cart', 'pending'))
          GROUP BY orders.id, orders.status;"
        )
        .bind(order_id)
        .bind(account_id)
        .fetch_one(&self.connection).await
        {
          Ok(row) => row,
          Err(e) => return Err(Error::database_query_error(e))
        };

      let items = match sqlx::query(
        "SELECT *
        FROM item
        WHERE order_id = $1;"
      )
      .bind(order_id)
      .map(|row| Item {
        id: row.get("id"),
        account_id,
        order_id: row.get("order_id"),
        restaurant_id: row.get("restaurant_id"),
        food_id: row.get("food_id"),
        quantity: row.get("quantity"),
        name: row.get("name"),
        image: row.get("image"),
        price: row.get("price"),
        discount_price: row.get("discount_price"),
      })
      .fetch_all(&self.connection).await        
      {
        Ok(row) => row,
        Err(e) => return Err(Error::database_query_error(e))
      };

      let result = GetOrder {
        items,
        account_id,
        order_id,
        status: order.get("status"),
        total_price: order.get("total_price"),
        total_discounted_price: order.get("total_discounted_price"),
      };
      Ok(result)
    }

    // we check to see if there is a cart with status 'cart' 
    // if exist return it, if not make a new one
    // it doesnt matter if this function is called multiple times or once in the frontend
    // we cant have 2 carts with 'cart' status
    pub async fn create_cart(&self, account_id: Uuid) -> Result<i32, Error> {
        match sqlx::query_scalar::<_, i32>(
          "SELECT id FROM orders WHERE account_id = $1 AND status = 'cart' LIMIT 1;"
        )
        .bind(account_id)
        .fetch_one(&self.connection)
        .await
        {
            Ok(id) => Ok(id),
            Err(_) => {
              match sqlx::query(
                "INSERT INTO orders (account_id, status, total_price, total_discounted_price) VALUES ($1, 'cart', 0, 0) RETURNING id;"
              )
              .bind(account_id)
              .map(|row| {
                row.get("id")
              })
              .fetch_one(&self.connection)
              .await
              {
                Ok(id) => Ok(id),
                Err(e) => Err(Error::database_query_error(e))
              }
            }
        }
    }

    // we insert an item
    // if it exist we update "quantity" based on the given count 
    // after each insert we check the "quantity", if it reaches 0 we delete the item
    // i dont think its necessary to add role checks to this, so user can use cart freely
    // we need to require auth for payment section
    pub async fn add_to_cart(&self, account_id: Uuid, item: NewItem) -> Result<bool, Error> {
      let mut tx = self.connection.begin().await.map_err(Error::database_query_error)?;
      sqlx::query(
          "INSERT INTO item (
              order_id, account_id, restaurant_id, food_id,
              quantity, name, image, price, discount_price
          )
          VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
          ON CONFLICT (order_id, food_id)
          DO UPDATE SET quantity = item.quantity + EXCLUDED.quantity"
      )
      .bind(item.order_id)
      .bind(account_id)
      .bind(item.restaurant_id)
      .bind(item.food_id)
      .bind(item.quantity)
      .bind(item.name)
      .bind(item.image)
      .bind(item.price)
      .bind(item.discount_price)
      .execute(&mut *tx)
      .await
      .map_err(Error::database_query_error)?;

      sqlx::query(
          "DELETE FROM item WHERE order_id = $1 AND food_id = $2 AND quantity = 0"
      )
      .bind(item.order_id)
      .bind(item.food_id)
      .execute(&mut *tx)
      .await
      .map_err(Error::database_query_error)?;
  
      tx.commit().await.map_err(Error::database_query_error)?;
  
      Ok(true)
  }
  


}