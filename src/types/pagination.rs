use std::{collections::HashMap, str::FromStr};

use crate::error::Error;

// # Pagination:
// this struct extract query paramaeters
// Extract query parameters from the `/restaurants` route
// `/restaurants?start=1&end=10`
#[derive(Debug, Default)]
pub struct Pagination {
    pub limit: Option<i32>,
    pub offset: i32,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("limit") && params.contains_key("offset") {
        Ok(Pagination {
            limit: Some(
                params
                    .get("limit")
                    .unwrap()
                    .parse::<i32>()
                    .map_err(|_| Error::parse_error)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(|_| Error::parse_error)?,
        })
    } else {
        Err(Error::missing_parameters)
    }
}

pub fn extract_params<T: FromStr>(params: &HashMap<String, String>, key: &str) -> Result<T, Error> {
    params
        .get(key)
        .ok_or_else(|| Error::missing_parameters) 
        .and_then(|s| s.parse().map_err(|_| Error::parse_error)) 
}
