use uuid::Uuid;

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
                panic!("Error writing initial schema: {}", Error::database_query_error(e));
            }
        }
    }

    pub async fn reset_sql(&self) {
        let sql = reset_sql();
        match sqlx::raw_sql(&sql).execute(&self.connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error reseting: {:?}", e);
                panic!("Error reseting: {}", Error::database_query_error(e));
            }
        }
    }

    pub async fn insert_sample_data(&self) {
        let sql = sample_data_sql();
        match sqlx::raw_sql(&sql).execute(&self.connection).await {
            Ok(_) => {}
            Err(e) => {
                tracing::error!("Error inserting fake data: {:?}", e);
                panic!(
                    "Error inserting fake data: {}",
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

