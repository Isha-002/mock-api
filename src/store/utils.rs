use std::fs::File;
use std::io::Write;

use uuid::Uuid;
use crate::utils::colors::ansi::*;
use crate::{error::Error, types::account::Role, utils::initial_sql::{init_sql, reset_sql, sample_data_sql}};

use super::Store;

#[macro_export]
macro_rules! verify_roles {
    ($self:expr, $id:expr, $($role:expr),+) => {{
        async {
            $(
                if $self.verify_role($id, $role).await? {
                    return Ok(true);
                }
            )+
            Ok(false)
        }
    }};
}

impl Store {
    pub async fn init_sql(&self) {
        let sql = init_sql();
        match sqlx::raw_sql(&sql).execute(&self.connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error writing initial schema: {:?}", e);
                println!("{BRIGHT_RED}Error writing initial schema:{RESET} {}", Error::database_query_error(e));
                println!("{BRIGHT_YELLOW}Writing the schema in a file...{RESET}");
                let file = File::create("0v1_initial_schema.sql");
                match file {
                    Ok(mut f) => {
                        f.write_all(sql.as_bytes()).unwrap_or_else(|e| println!("{BRIGHT_RED}We could not write data into the file:{RESET} {e}"))
                    },
                    Err(e) => println!("{BRIGHT_RED}We could not create the initial database schema file:{RESET} {e}")
                }
            }
        }
    }

    pub async fn reset_sql(&self) {
        let sql = reset_sql();
        match sqlx::raw_sql(&sql).execute(&self.connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error reseting: {:?}", e);
                println!("{BRIGHT_RED}Error reseting:{RESET} {}", Error::database_query_error(e));
                println!("{BRIGHT_YELLOW}Writing the reset schema in a file...{RESET}");
                let file = File::create("0v1_reset_schema.sql");
                match file {
                    Ok(mut f) => {
                        f.write_all(sql.as_bytes()).unwrap_or_else(|e| println!("{BRIGHT_RED}We could not write data into the file:{RESET} {e}"))
                    },
                    Err(e) => println!("{BRIGHT_RED}We could not create the reset schema file:{RESET} {e}")
                }
            }
        }
    }

    // we are paased the stage to use this function
    pub async fn insert_sample_data(&self) {
        let sql = sample_data_sql();
        match sqlx::raw_sql(&sql).execute(&self.connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error inserting sample data: {:?}", e);
                println!(
                    "{BRIGHT_RED}Error inserting sample data:{RESET} {}",
                    Error::database_query_error(e)
                );
            }
        }
    }

    pub async fn verify_role(&self, id: &Uuid, role: Role) -> Result<bool, Error> {
        match sqlx::query(
            "SELECT * FROM account
            WHERE id = $1 AND role = $2"
        )
        .bind(id)
        .bind(role)
        .fetch_optional(&self.connection)
        .await
        {
            Ok(r) => Ok(r.is_some()),
            Err(e) => {
                tracing::event!(tracing::Level::ERROR, "{:?}", e);
                Err(Error::database_query_error(e))
            }
        }
    }
}

