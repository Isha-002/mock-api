use crate::{
    error::{self, Error},
    types::{
        food::Food,
        restaurant::{self, NewRestaurant, Restaurant, RestaurantId},
    },
};
use sqlx::Row;
use sqlx::{
    postgres::{PgPoolOptions, PgRow},
    PgPool,
};

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(10)
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
    ) -> Result<Vec<Restaurant>, error::Error> {
        match sqlx::query("SELECT * from restaurant LIMIT $1 OFFSET $2")
            .bind(limit)
            .bind(offset)
            .map(|row: PgRow| Restaurant {
                id: RestaurantId(row.get("id")),
                name: row.get("name"),
                rating: row.get("rating"),
                distance: row.get("distance"),
                tags: row.get("tags"),
                menu: row.get("menu"),
                image: row.get("image"),
            })
            .fetch_all(&self.connection)
            .await
        {
            Ok(restaurants) => Ok(restaurants),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::database_query_error(e))
            }
        }
    }
    pub async fn add_restaurant(
        &self,
        new_restaurant: NewRestaurant,
    ) -> Result<Restaurant, error::Error> {
        match sqlx::query(
            "INSERT INTO restaurant (name, rating, distance, tags, menu, image)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id, name, rating, distance, tags, menu, image
        ",
        )
        .bind(new_restaurant.name)
        .bind(new_restaurant.rating)
        .bind(new_restaurant.distance)
        .bind(new_restaurant.tags)
        .bind(new_restaurant.menu)
        .bind(new_restaurant.image)
        .map(|row| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            menu: row.get("menu"),
            image: row.get("image"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn update_restaurant(
        &self,
        restaurant: Restaurant,
        restaurant_id: i32,
    ) -> Result<Restaurant, error::Error> {
        match sqlx::query(
            "UPDATE restaurant 
        SET name = $1, rating = $2, distance = $3, tags = $4, menu = $5, image = $6 
        WHERE id = $7
        RETURNING id, name, rating, distance, tags, menu, image
        ",
        )
        .bind(restaurant.name)
        .bind(restaurant.rating)
        .bind(restaurant.distance)
        .bind(restaurant.tags)
        .bind(restaurant.menu)
        .bind(restaurant.image)
        .bind(restaurant_id)
        .map(|row| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            menu: row.get("menu"),
            image: row.get("image"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
    pub async fn delete_restaurant(&self, restaurant_id: i32) -> Result<bool, error::Error> {
        match sqlx::query("DELETE FROM restaurant WHERE id = $1")
            .bind(restaurant_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
    pub async fn get_single_restaurant(&self, restaurant_id: i32) -> Result<Restaurant, error::Error> {
        match sqlx::query(
            "SELECT * FROM restaurant
            WHERE id = $1"
        )
        .bind(restaurant_id)
        .map(|row: PgRow| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            menu: row.get("menu"),
            image: row.get("image"),
        })
        .fetch_one(&self.connection)
        .await 
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
}
