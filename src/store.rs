use crate::{
    error::{self, Error},
    types::{
        comment::{Comment, NewComment}, food::Food, restaurant::{self, NewRestaurant, Restaurant, RestaurantId}
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
                address: row.get("address"),
                city: row.get("city"),
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
            "INSERT INTO restaurant (name, rating, distance, tags, menu, image, address, city)
        VALUES ($1, $2, $3, $4, $5::foodType[], $6, $7, $8)
        RETURNING id, name, rating, distance, tags, menu, image, address, city
        ",
        )
        .bind(new_restaurant.name)
        .bind(new_restaurant.rating)
        .bind(new_restaurant.distance)
        .bind(new_restaurant.tags)
        .bind(new_restaurant.menu)
        .bind(new_restaurant.image)
        .bind(new_restaurant.address)
        .bind(new_restaurant.city)
        .map(|row| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            menu: row.get("menu"),
            image: row.get("image"),
            address: row.get("address"),
            city: row.get("city")
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
        SET name = $1, rating = $2, distance = $3, tags = $4, menu = $5, image = $6, address = $7, city = $8
        WHERE id = $9
        RETURNING id, name, rating, distance, tags, menu, image, address, city
        ",
        )
        .bind(restaurant.name)
        .bind(restaurant.rating)
        .bind(restaurant.distance)
        .bind(restaurant.tags)
        .bind(restaurant.menu)
        .bind(restaurant.image)
        .bind(restaurant.address)
        .bind(restaurant.city)
        .bind(restaurant_id)

        .map(|row| Restaurant {
            id: RestaurantId(row.get("id")),
            name: row.get("name"),
            rating: row.get("rating"),
            distance: row.get("distance"),
            tags: row.get("tags"),
            menu: row.get("menu"),
            image: row.get("image"),
            address: row.get("address"),
            city: row.get("city"),
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
            address: row.get("address"),
            city: row.get("city"),
        })
        .fetch_one(&self.connection)
        .await 
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn get_comments(&self, restaurant_id: i32) -> Result<Vec<Comment>, error::Error> {
        match sqlx::query(
            "SELECT * FROM comments WHERE restaurant_id = $1 
            ORDER BY likes DESC"
        ).bind(restaurant_id)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_all(&self.connection)
        .await
    {
        Ok(comments) => Ok(comments),
        Err(e) => Err(error::Error::database_query_error(e)),
    }
    }
    pub async fn add_comment(&self, restaurant_id: i32, comment: NewComment) -> Result<Comment, error::Error> {
        match sqlx::query(
            "INSERT INTO comments (restaurant_id name text) VALUES ($1, $2, $3) 
            WHERE id = $4
            RETURNING *"
        )
        .bind(restaurant_id) 
        .bind(comment.name) 
        .bind(comment.text) 
        .bind(restaurant_id) 
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comments) => Ok(comments),
            Err(e) => Err(error::Error::database_query_error(e)),
        }
    }
    pub async fn delete_comment(&self, restaurant_id: i32, comment_id: i32) -> Result<bool, error::Error> {
        match sqlx::query("DELETE FROM comments WHERE restaurant_id = $1 AND id = $2")
        .bind(restaurant_id)
        .bind(comment_id)
        .execute(&self.connection)
        .await
    {
        Ok(_) => Ok(true),
        Err(e) => Err(Error::database_query_error(e)),
    }
    }

    pub async fn add_comment_like(&self, restaurant_id: i32, comment_id: i32) -> Result<Comment, error::Error> {
        match sqlx::query(
            "UPDATE comments SET likes = likes + 1 WHERE restaurant_id = $1 AND id = $2
            RETURNING *"
        )
        .bind(restaurant_id)
        .bind(comment_id)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comment) => Ok(comment),
            Err(e) => Err(error::Error::database_query_error(e)),
        }
    }

    pub async fn remove_comment_like(&self, restaurant_id: i32, comment_id: i32) -> Result<Comment, error::Error> {
        match sqlx::query(
            "UPDATE comments SET likes = GREATEST(likes - 1, 0) WHERE restaurant_id = $1 AND id = $2 AND likes > 0
            RETURNING *"
        )
        .bind(restaurant_id)
        .bind(comment_id)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comment) => Ok(comment),
            Err(e) => Err(error::Error::database_query_error(e)),
        }
    }

    pub async fn add_comment_dislike(&self, restaurant_id: i32, comment_id: i32) -> Result<Comment, error::Error> {
        match sqlx::query(
            "UPDATE comments SET dislikes = dislikes + 1 WHERE restaurant_id = $1 AND id = $2
            RETURNING *"
        )
        .bind(restaurant_id)
        .bind(comment_id)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comment) => Ok(comment),
            Err(e) => Err(error::Error::database_query_error(e)),
        }
    }

    pub async fn remove_comment_dislike(&self, restaurant_id: i32, comment_id: i32) -> Result<Comment, error::Error> {
        match sqlx::query(
            "UPDATE comments SET dislikes = GREATEST(dislikes - 1, 0) WHERE restaurant_id = $1 AND id = $2 AND dislikes > 0
            RETURNING *"
        )
        .bind(restaurant_id)
        .bind(comment_id)
        .map(|row: PgRow| Comment {
            id: row.get("id"),
            restaurant_id,
            name: row.get("name"),
            text: row.get("text"),
            likes: row.get("likes"),
            dislikes: row.get("dislikes")
        })
        .fetch_one(&self.connection)
        .await
        {
            Ok(comment) => Ok(comment),
            Err(e) => Err(error::Error::database_query_error(e)),
        }
    }
}





