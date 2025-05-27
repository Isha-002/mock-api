use super::Store;
use crate::{
    error::Error,
    types::{account::Role, restaurant::OpenHours},
    verify_roles,
};
use sqlx::Row;
use uuid::Uuid;

impl Store {
    pub async fn get_restaurant_hours(&self, restaurant_id: i32) -> Result<Vec<OpenHours>, Error> {
        match sqlx::query(
            "SELECT day_of_week, open_time, close_time FROM restaurant_hours
            WHERE restaurant_id = $1
            ORDER BY day_of_week; ",
        )
        .bind(restaurant_id)
        .map(|row| OpenHours {
            restaurant_id: row.get("restaurant_id"),
            day_of_week: row.get("day_of_week"),
            open_time: row.get("open_time"),
            close_time: row.get("close_time"),
        })
        .fetch_all(&self.connection)
        .await
        {
            Ok(hours) => Ok(hours),
            Err(e) => Err(Error::database_query_error(e)),
        }
    }

    pub async fn post_restaurant_hours(
        &self,
        account_id: Uuid,
        hours: OpenHours,
    ) -> Result<OpenHours, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
            Ok(true) => {
                match sqlx::query(
                "INSERT INTO restaurant_hours (restaurant_id, day_of_week, open_time, close_time)
                VALUES ($1, $2, $3, $4);"
              )
              .bind(hours.restaurant_id)
              .bind(hours.day_of_week)
              .bind(hours.open_time)
              .bind(hours.close_time)
              .map(|row| OpenHours {
                restaurant_id: row.get("restaurant_id"),
                day_of_week: row.get("day_of_week"),
                open_time: row.get("open_time"),
                close_time: row.get("close_time"),
            })
            .fetch_one(&self.connection)
            .await
            {
                Ok(hours) => Ok(hours),
                Err(e) => Err(Error::database_query_error(e)),
            }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }

    pub async fn put_restaurant_hours(
        &self,
        account_id: Uuid,
        hours: OpenHours,
    ) -> Result<OpenHours, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
            Ok(true) => {
                match sqlx::query(
                    "UPDATE restaurant_hours
                    SET open_time = $1, close_time = $2
                    WHERE restaurant_id = $3
                    AND day_of_week = $4;",
                )
                .bind(hours.open_time)
                .bind(hours.close_time)
                .bind(hours.restaurant_id)
                .bind(hours.day_of_week)
                .map(|row| OpenHours {
                    restaurant_id: row.get("restaurant_id"),
                    day_of_week: row.get("day_of_week"),
                    open_time: row.get("open_time"),
                    close_time: row.get("close_time"),
                })
                .fetch_one(&self.connection)
                .await
                {
                    Ok(hours) => Ok(hours),
                    Err(e) => Err(Error::database_query_error(e)),
                }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }

    pub async fn delete_restaurant_hours(
        &self,
        account_id: Uuid,
        hours: OpenHours,
    ) -> Result<bool, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
            Ok(true) => {
                match sqlx::query(
                    "DELETE FROM restaurant_hours 
                    WHERE restaurant_id = $1
                    AND day_of_week = $2;",
                )
                .bind(hours.restaurant_id)
                .bind(hours.day_of_week)
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
}
