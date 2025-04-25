use std::collections::HashMap;

use crate::error::Error;

/// # Paginatio:
/// this struct extract query paramaeters
/// Extract query parameters from the `/restaurants` route
/// `/restaurants?start=1&end=10`
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
                    .map_err(Error::parse_error)?,
            ),
            offset: params
                .get("offset")
                .unwrap()
                .parse::<i32>()
                .map_err(Error::parse_error)?,
        })
    } else {
        Err(Error::missing_parameters)
    }
}
