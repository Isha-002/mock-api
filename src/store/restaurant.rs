use crate::{
    error::Error,
    types::restaurant::{NewRestaurant, Restaurant, RestaurantId},
};
use sqlx::postgres::PgRow;
use sqlx::Row;

use super::Store;

impl Store {
    pub async fn get_restaurants(
        &self,
        // if we don’t pass a number, "limit" will be None, and PostgreSQL will ignore it
        limit: Option<i32>,
        // if we pass 0 as an offset, it will do the same
        offset: i32,
    ) -> Result<Vec<Restaurant>, Error> {
        
        // println!("limit = {:?}, offset = {}", limit, offset);
        match sqlx::query(
            "SELECT 
            restaurant.*, 
            COALESCE(AVG(comments.rating), 0) AS average_rating
            FROM restaurant
            LEFT JOIN comments 
            ON restaurant.id = comments.restaurant_id
            GROUP BY restaurant.id
            LIMIT $1 OFFSET $2;",
        )
        .bind(limit)
        .bind(offset)
        .map(|row: PgRow| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("average_rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            image: row.get("image"),
            address: row.get("address"),
            city: row.get("city"),
            location: row.get("location"),
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
    pub async fn add_restaurant(&self, new_restaurant: NewRestaurant) -> Result<Restaurant, Error> {
        match sqlx::query(
            "INSERT INTO restaurant (name, rating, distance, tags, image, address, city, location)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, name, rating, distance, tags, image, address, city, location
            ",
        )
        .bind(new_restaurant.name)
        .bind(new_restaurant.rating)
        .bind(new_restaurant.distance)
        .bind(new_restaurant.tags)
        .bind(new_restaurant.image)
        .bind(new_restaurant.address)
        .bind(new_restaurant.city)
        .bind(new_restaurant.location)
        .map(|row| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            image: row.get("image"),
            address: row.get("address"),
            city: row.get("city"),
            location: row.get("location"),
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
    ) -> Result<Restaurant, Error> {
        match sqlx::query(
        "UPDATE restaurant 
        SET name = $1, rating = $2, distance = $3, tags = $4, image = $5, address = $6, city = $7, location = $8
        WHERE id = $9
        RETURNING id, name, rating, distance, tags, image, address, city, location
        ",
    )
    .bind(restaurant.name)
    .bind(restaurant.rating)
    .bind(restaurant.distance)
    .bind(restaurant.tags)
    .bind(restaurant.image)
    .bind(restaurant.address)
    .bind(restaurant.city)
    .bind(restaurant.location)
    .bind(restaurant_id)

    .map(|row| Restaurant {
        id: RestaurantId(row.get("id")),
        name: row.get("name"),
        rating: row.get("rating"),
        distance: row.get("distance"),
        tags: row.get("tags"),
        image: row.get("image"),
        address: row.get("address"),
        city: row.get("city"),
        location: row.get("location")
    })
    .fetch_one(&self.connection)
    .await
    {
        Ok(restaurant) => Ok(restaurant),
        Err(e) => Err(Error::database_query_error(e)),
    }
    }
    pub async fn delete_restaurant(&self, restaurant_id: i32) -> Result<bool, Error> {
        match sqlx::query("DELETE FROM restaurant WHERE id = $1")
            .bind(restaurant_id)
            .execute(&self.connection)
            .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
    pub async fn get_single_restaurant(&self, restaurant_id: i32) -> Result<Restaurant, Error> {
        match sqlx::query(
            "SELECT * FROM restaurant
        WHERE id = $1",
        )
        .bind(restaurant_id)
        .map(|row: PgRow| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            image: row.get("image"),
            address: row.get("address"),
            city: row.get("city"),
            location: row.get("location"),
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn insert_file_to_restaurant(
        &self,
        url: &str,
        restaurant_id: i32,
    ) -> Result<bool, Error> {
        match sqlx::query(
            "UPDATE restaurant 
            SET image = $1
            WHERE id = $2",
        )
        .bind(url)
        .bind(restaurant_id)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn _get_restaurant_pfp_image(&self, restaurant_id: i32) -> Result<String, Error> {
        match sqlx::query(
            "SELECT image from restaurant
            WHERE id = $1",
        )
        .bind(restaurant_id)
        .map(|row: PgRow| row.get("image"))
        .fetch_one(&self.connection)
        .await
        {
            Ok(url) => Ok(url),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
}
