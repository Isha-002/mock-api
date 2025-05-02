use sqlx::error::Error as SqlxError;
use std::fmt::{self};
use tracing::{event, Level};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    reject::{Reject, Rejection},
    reply::Reply,
};

const DUPLICATE_KEY: &str = "23505";

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    parse_error(std::num::ParseIntError),
    missing_parameters,
    database_query_error(SqlxError),
    creating_upload_dir(std::io::Error),
    write_file(std::io::Error),
    no_file,
    wrong_password,
    argon_library_error(argon2::Error),
    missing_email_or_phone,
    bail_out_card,
    invalid_error_code(String),
    failed_to_get_account(SqlxError)
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::parse_error(ref err) => write!(f, "Cannot parse parameters: {err}"),
            Error::missing_parameters => write!(f, "Missing parameters"),
            Error::database_query_error(e) => write!(f, "Database error: {e}"),
            Error::creating_upload_dir(e) => write!(f, "Failed to create upload folder: {e}"),
            Error::write_file(e) => write!(f, "Failed to write file: {e}"),
            Error::no_file => write!(f, "No file provided"),
            Error::wrong_password => write!(f, "Incorrect password"),
            Error::argon_library_error(e) => write!(f, "Password verification error: {e}"),
            Error::missing_email_or_phone => write!(f, "Please provide email or phone number"),
            Error::bail_out_card => write!(f, "Unexpected error occurred"),
            Error::invalid_error_code(code) => write!(f, "Invalid database error code: {code}"),
            Error::failed_to_get_account(e) => write!(f, "Failed to login/get account: {e}"),
        }
    }
}

impl Reject for Error {}

#[derive(Debug)]
pub struct InvalidID;
impl Reject for InvalidID {}

impl fmt::Display for InvalidID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid ID format - must be a valid UUID or integer")
    }
}

pub async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    if let Some(Error::database_query_error(e)) = r.find() {
        match e {
            sqlx::Error::Database(err) => {
                let error_code = err.code().unwrap_or_default();

                event!(
                    Level::DEBUG,
                    "Database error: code={}, message={}",
                    error_code,
                    err.message()
                );

                if error_code == DUPLICATE_KEY {
                    Ok(warp::reply::with_status(
                        "Account already exists".to_string(),
                        warp::http::StatusCode::CONFLICT,
                    ))
                } else {
                    Ok(warp::reply::with_status(
                        format!("Database error: {}", err.message()),
                        warp::http::StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
            _ => Ok(warp::reply::with_status(
                "Database operation failed".to_string(),
                warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            )),
        }
    } else if let Some(Error::wrong_password) = r.find() {
        event!(
            Level::WARN,
            "Authentication attempt with incorrect password"
        );
        Ok(warp::reply::with_status(
            "Wrong credentials".to_string(),
            warp::http::StatusCode::UNAUTHORIZED,
        ))
    } else if let Some(error) = r.find::<Error>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(error) = r.find::<InvalidID>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else if let Some(error) = r.find::<BodyDeserializeError>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::BAD_REQUEST,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Endpoint not found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}
