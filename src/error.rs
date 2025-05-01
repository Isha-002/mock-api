use sqlx::error::Error as SqlxError;
use std::fmt::{self};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    reject::{Reject, Rejection},
    reply::Reply,
};

// duplicate key value error code:
const DUPLICATE_KEY: u32 = 23505;

/// # How errors work
/// 1. adding an specific error to the enum Error
/// 2. create a `fmt::Display` implementation for that enum
/// 3. return a `write!` macro with your custom error message

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    parse_error(std::num::ParseIntError),
    missing_parameters,
    // unacceptable_parameters,
    // restaurant_not_found,
    // unkown_error,
    database_query_error(SqlxError),
    creating_upload_dir(std::io::Error),
    write_file(std::io::Error),
    no_file,
    bail_out_card,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::parse_error(ref err) => {
                write!(f, "cannot parse parameters: {err}")
            }
            Error::missing_parameters => {
                write!(f, "missing parameters")
            }
            // Error::unacceptable_parameters => {
            //     write!(f, "parameters are not acceptable")
            // }
            // Error::restaurant_not_found => {
            //     write!(f, "restaurant not found")
            // }
            // Error::unkown_error => {
            //     write!(f, "something happened and we dont know what!")
            // }
            Error::database_query_error(e) => {
                write!(f, "could not execute query: {}", e)
            }
            Error::creating_upload_dir(e) => {
                write!(f, "failed to create upload folder: {}", e)
            }
            Error::write_file(e) => {
                write!(f, "failed to write the file: {}", e)
            }
            Error::no_file => {
                write!(f, "found no file")
            }
            Error::bail_out_card => {
                write!(f, "well, we found an error but dont know why!\n this happens because i had a hard time handling errors and i wanted to bail out")
            }
        }
    }
}

impl Reject for Error {}

#[derive(Debug)]
pub struct InvalidID;
impl Reject for InvalidID {}

impl fmt::Display for InvalidID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid id")
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::database_query_error(e)) = r.find() {
        match e {
            sqlx::Error::Database(err) => {
                if err.code().unwrap().parse::<u32>().unwrap() == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exsists".to_string(),
                        warp::http::StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        "Cannot update data".to_string(),
                        warp::http::StatusCode::UNPROCESSABLE_ENTITY,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Cannot update data".to_string(),
                warp::http::StatusCode::UNPROCESSABLE_ENTITY,
            )),
        }
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<InvalidID>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
