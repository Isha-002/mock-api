use std::collections::HashMap;

use crate::error::Error;

/// # Paginatio:
/// this struct extract query paramaeters
/// Extract query parameters from the `/restaurants` route
/// `/restaurants?start=1&end=10`
#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::parse_error)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Error::parse_error)?,
        })
    } else {
        Err(Error::missing_parameters)
    }
}
