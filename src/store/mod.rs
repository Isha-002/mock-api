use sqlx::{postgres::PgPoolOptions, PgPool};


pub mod restaurant;
pub mod comment;
pub mod auth;
pub mod search;
pub mod utils;
pub mod food;
pub mod order;
pub mod owner;
pub mod payment;
pub mod open_hours;






#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(50)
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => panic!("couldn't establish a database connection: {e:?}"),
        };
        Store {
            connection: db_pool,
        }
    }
}