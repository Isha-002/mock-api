use crate::{
    error::Error,
    types::account::{Account, Login, Role},
};
use sqlx::postgres::PgRow;
use sqlx::Row;
use uuid::Uuid;

use super::Store;

impl Store {
    pub async fn add_account(&self, account: &Account) -> Result<bool, Error> {
        let account = account.clone();
        match sqlx::query(
            "INSERT INTO account (id, name, email, password, phone_number, role)
        VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(account.id)
        .bind(account.name)
        .bind(account.email)
        .bind(account.password)
        .bind(account.phone_number)
        .bind(account.role)
        .execute(&self.connection)
        .await
        {
            Ok(_) => Ok(true),
            Err(e) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = e
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = e.as_database_error().unwrap().message(),
                    constraint = e.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::database_query_error(e))
            }
        }
    }

    pub async fn get_account(&self, login: &Login) -> Result<Account, Error> {
        let (query, value) = 
        if let Some(phone) = &login.phone_number {
            ("SELECT * FROM account WHERE phone_number = $1", phone)
        } else if let Some(email) = &login.email {
            ("SELECT * FROM account WHERE email = $1", email)
        } else {
            return Err(Error::missing_email_or_phone);
        };

        match sqlx::query(query)
            .bind(value)
            .map(|row: PgRow| Account {
                id: row.get("id"),
                name: row.get("name"),
                email: row.get("email"),
                phone_number: row.get("phone_number"),
                password: row.get("password"),
                role: row.get("role"),
            })
            .fetch_one(&self.connection)
            .await
        {
            Ok(account) => Ok(account),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::failed_to_get_account(e))
            }
        }
    }

    pub async fn verify_restaurant_modification_access(&self, restaurant_id: i32, modifier_id: &Uuid) -> Result<bool, Error> {

        let is_admin =  self.verify_role(modifier_id, Role::admin).await;

        match is_admin {
            Ok(true) => Ok(true),
            Ok(false) => {
                match sqlx::query(
                    "SELECT 1 FROM owner 
                    WHERE restaurant_id = $1 AND account_id = $2;"
                )
                .bind(restaurant_id)
                .bind(modifier_id)
                .fetch_optional(&self.connection)
                .await
                {
                    Ok(r) => Ok(r.is_some()),
                    Err(e) => {
                        tracing::event!(tracing::Level::ERROR, "{:?}", e);
                        Err(Error::database_query_error(e))
                    }
                }
            },
            Err(e) => Err(e)
        }
    }
}
