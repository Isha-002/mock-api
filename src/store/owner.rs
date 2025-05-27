use crate::{
    error::Error,
    types::{
        account::Role,
        owner::{GetOwner, NewOwner, Owner},
    },
    verify_roles,
};
use sqlx::Row;
use uuid::Uuid;

use super::Store;

// we dont have "delete" because "owner" gets deleted automatically when the restaurant is deleted
impl Store {
    // we give user a "restaurant_owner" role.
    // we take national_id when the user creates restaurant
    // "create_restaurant" endpoint must return "restaurant_id"
    // we call the endpoint associated with "create_owner" function
    // we bind the owner info to his restaurant_id
    // later we can store more data as needed for the owner
    pub async fn create_owner(&self, owner: NewOwner, account_id: Uuid) -> Result<(), Error> {
        match verify_roles!(
            &self,
            &account_id,
            Role::admin,
            Role::restaurant_owner
        )
        .await
        {
            Ok(true) => {
                match sqlx::query(
                    "INSERT INTO owner (restaurant_id, account_id, national_id)
                  VALUES ($1, $2, $3);",
                )
                .bind(owner.restaurant_id)
                .bind(account_id)
                .bind(owner.national_id)
                .execute(&self.connection)
                .await
                {
                    Ok(_) => Ok(()),
                    Err(e) => Err(Error::database_query_error(e)),
                }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }

    pub async fn get_owner(&self, account_id: Uuid) -> Result<GetOwner, Error> {
        match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
            Ok(true) => {
                match sqlx::query(
                    "
                SELECT 
                  owner.national_id,
                  account.name,
                  account.email,
                  account.phone_number
                FROM owner
                JOIN account ON owner.account_id = account.id
                WHERE owner.account_id = $1;
                ",
                )
                .bind(account_id)
                .map(|row| GetOwner {
                    name: row.get("name"),
                    phone_number: row.get("phone_number"),
                    email: row.get("email"),
                    national_id: row.get("national_id"),
                })
                .fetch_one(&self.connection)
                .await
                {
                    Ok(owner) => Ok(owner),
                    Err(e) => Err(Error::database_query_error(e)),
                }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }

    pub async fn update_owner_national_id(&self, owner: Owner, account_id: Uuid) -> Result<String, Error> {
        match verify_roles!(
            &self,
            &account_id,
            Role::admin,
            Role::restaurant_owner
        )
        .await
        {
            Ok(true) => {
                match sqlx::query(
                    "UPDATE owner
            SET national_id = $1
            WHERE account_id = $2 AND restaurant_id = $3
            RETURNING national_id;",
                )
                .bind(owner.national_id)
                .bind(owner.account_id)
                .bind(owner.restaurant_id)
                .map(|row| row.get("national_id"))
                .fetch_one(&self.connection)
                .await
                {
                    Ok(id) => Ok(id),
                    Err(e) => Err(Error::database_query_error(e)),
                }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }

    pub async fn replace_owner(&self, owner: Owner, account_id: Uuid) -> Result<Owner, Error> {
        match verify_roles!(
            &self,
            &account_id,
            Role::admin,
            Role::restaurant_owner
        )
        .await
        {
            Ok(true) => {
                match sqlx::query(
                    "UPDATE owner
            SET account_id = $1, national_id = $2
            WHERE restaurant_id = $3
            RETURNING *;",
                )
                .bind(owner.account_id)
                .bind(owner.national_id)
                .bind(owner.restaurant_id)
                .map(|row| Owner {
                    restaurant_id: row.get("restaurant_id"),
                    account_id: row.get("account_id"),
                    national_id: row.get("national_id"),
                })
                .fetch_one(&self.connection)
                .await
                {
                    Ok(owner) => Ok(owner),
                    Err(e) => Err(Error::database_query_error(e)),
                }
            }
            Ok(false) => Err(Error::not_authorized),
            Err(e) => Err(e),
        }
    }
}

// match verify_roles!(&self, &account_id, Role::admin, Role::restaurant_owner).await {
//   Ok(true) => {

//   }
//   Ok(false) => Err(Error::not_authorized),
//   Err(e) => Err(e),
// }
