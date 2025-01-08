use std::fmt::{self};
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
    reject::{Reject, Rejection},
    reply::Reply,
};

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Error {
    parse_error(std::num::ParseIntError),
    missing_parameters,
    unacceptable_parameters,
    restaurant_not_found,
    unkown_error,
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::parse_error(ref err) => {
                write!(f, "cannot parse parameters: {err}")
            }
            Error::missing_parameters => {
                write!(f, "missing parameters")
            }
            Error::unacceptable_parameters => {
                write!(f, "parameters are not acceptable")
            }
            Error::restaurant_not_found => {
                write!(f, "restaurant not found")
            }
            Error::unkown_error => {
                write!(f, "something happened and we dont know what!")
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
    println!("{:?}", r);

    if let Some(error) = r.find::<Error>() {
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
