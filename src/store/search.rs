use crate::{
    error::Error,
    types::restaurant::{Restaurant, RestaurantId},
};
use sqlx::postgres::PgRow;
use sqlx::Row;
use urlencoding::decode;

use super::Store;

impl Store {
    pub async fn search_by_city(&self, city: String) -> Result<Vec<Restaurant>, Error> {
        let decoded_city = decode(&city).unwrap().into_owned();
        match sqlx::query(
            "SELECT * from restaurant
        WHERE city = $1
        ORDER BY rating DESC",
        )
        .bind(decoded_city)
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
        .fetch_all(&self.connection)
        .await
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn search_by_tag(&self, tag: String, city: String) -> Result<Vec<Restaurant>, Error> {
        let decoded_tag = decode(&tag).unwrap().into_owned();
        match sqlx::query(
            "SELECT * from restaurant
        WHERE $1 = ANY(tags) AND city = $2
        ORDER BY rating DESC",
        )
        .bind(decoded_tag)
        .bind(city)
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
        .fetch_all(&self.connection)
        .await
        {
            Ok(restaurant) => Ok(restaurant),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }
}
