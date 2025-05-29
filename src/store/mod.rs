use std::time::Duration;
use crate::utils::colors::ansi::*;
use sqlx::{postgres::PgPoolOptions, Error, PgPool};
use text_io::read;

pub mod auth;
pub mod comment;
pub mod food;
pub mod open_hours;
pub mod order;
pub mod owner;
pub mod payment;
pub mod restaurant;
pub mod search;
pub mod utils;

#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
            .max_connections(50)
            .acquire_timeout(Duration::from_secs(3))
            .connect(db_url)
            .await
        {
            Ok(pool) => pool,
            Err(e) => {
                match &e {
                    Error::Configuration(_) => {
                        loop {
                            println!("{BRIGHT_RED}Invalid connection string:{RESET} {e}");
                            println!("{BRIGHT_BLUE}Enter a valid connection string:{RESET}");
                            let connection_str: String = read!();
                            
                            
                            match PgPoolOptions::new()
                                .max_connections(50)
                                .acquire_timeout(Duration::from_secs(3))
                                .connect(&connection_str)
                                .await
                            {
                                Ok(pool) => break pool,
                                Err(new_e) => {
                                    println!("{BRIGHT_RED}Connection failed:{RESET} {}", new_e);
                                    continue;
                                }
                            }
                        }
                    },
                    _ => {
                        println!("{BRIGHT_RED}Unknown connection error:{RESET} {e:?}");
                        panic!("couldn't establish a database connection: {e:?}")
                    }
                }
            }
        };
        
        Store {
            connection: db_pool,
        }
    }
}
