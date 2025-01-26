use sqlx::{postgres::{PgPoolOptions, PgRow}, PgPool};
use sqlx::Row;
use crate::{error::Error, types::{food::Food, restaurant::{self, Restaurant, RestaurantId}}};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("coundln't establish a database connection: {e:?}"),
        };
        Store {
            connection: db_pool,
        }
    }

    pub async fn get_restaurants(
        &self,
        // if we don’t pass a number, "limit" will be None, and PostgreSQL will ignore it
        limit: Option<i32>,
        // if we pass 0 as an offset, it will do the same
        offset: i32,
    ) -> Result<Vec<Restaurant>, sqlx::Error> {
        match sqlx::query("SELECT * from restaurant LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Restaurant {
                id: RestaurantId(row.get("id")),
                name: row.get("name"),
                rating: row.get("rating"),
                distance: row.get("distance"),
                tags: row.get("tags"),
                menu: row.get::<Vec<Food>, _>("menu"),
                image: row.get("image")
            })
            .fetch_all(&self.connection).await {
            Ok(restaurants) => Ok(restaurants),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                // Err(Error::DatabaseQueryError)
                Err(e)
            }
        }
    }
}
