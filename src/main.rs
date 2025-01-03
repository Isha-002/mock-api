use core::option::Option::Some;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fmt::{self},
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::filters::path::param;
use warp::{
    filters::cors::CorsForbidden,
    http::Method,
    reject::{Reject, Rejection},
    reply::Reply,
    Filter,
};

#[derive(Debug)]
struct Pagination {
    start: usize,
    end: usize,
}

fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Errors> {
    if params.contains_key("start") && params.contains_key("end") {
        Ok(Pagination {
            start: params
                .get("start")
                .unwrap()
                .parse::<usize>()
                .map_err(Errors::parse_error)?,
            end: params
                .get("end")
                .unwrap()
                .parse::<usize>()
                .map_err(Errors::parse_error)?,
        })
    } else {
        Err(Errors::missing_parameters)
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)]
enum Errors {
    parse_error(std::num::ParseIntError),
    missing_parameters,
}
impl std::fmt::Display for Errors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Errors::parse_error(ref err) => {
                write!(f, "cannot parse parameters: {err}")
            }
            Errors::missing_parameters => {
                write!(f, "missing parameters")
            }
        }
    }
}

impl Reject for Errors {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Store {
    restaurants: HashMap<RestaurantId, Restaurant>,
}

impl Store {
    fn init() -> Self {
        let mock_data = include_str!("../mock_data.json");
        match serde_json::from_str(mock_data) {
            Ok(data) => Store { restaurants: data },
            Err(e) => {
                println!("there was an error reading mock_data.json: {e}");
                println!("initialized Empty...");
                Store::new()
            }
        }
    }
    fn new() -> Self {
        Store {
            restaurants: HashMap::new(),
        }
    }
    fn is_empty(&self) -> bool {
        self.restaurants.is_empty()
    }
}

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

#[derive(Serialize, Deserialize, Clone, Debug)]
struct Restaurant {
    id: RestaurantId,
    name: String,
    rating: f32,
    distance: f64,
    tags: Option<Vec<String>>,
    image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, Eq, Hash, PartialEq)]
struct RestaurantId(String);

impl Restaurant {
    fn new(
        id: RestaurantId,
        name: &str,
        rating: f32,
        distance: f64,
        tags: Option<Vec<String>>,
        image: &str,
    ) -> Self {
        Restaurant {
            id,
            name: name.to_string(),
            rating,
            distance,
            tags,
            image: image.to_string(),
        }
    }
    fn update_name(&self, new_name: &str) -> Self {
        Restaurant::new(
            self.id.clone(),
            new_name,
            self.rating,
            self.distance,
            self.tags.clone(),
            &self.image,
        )
    }
}

impl fmt::Display for Restaurant {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "`{{ id: {}, name: {}, rating: {}, distance: {}, tags: {:?}, image: {} }}`",
            self.id, self.name, self.rating, self.distance, self.tags, self.image
        )
    }
}

impl fmt::Display for RestaurantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for RestaurantId {
    type Err = std::io::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(RestaurantId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

#[tokio::main]
async fn main() {
    let store = Store::init();
    // println!("{store:?}");
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    // let home = warp::path("/").map(|| "home".to_string());
    let res = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter)
        .and_then(get_restaurants)
        .recover(return_error);
    println!("starting the server on http://localhost:4444/");
    warp::serve(res.with(cors)).run(([0, 0, 0, 0], 4444)).await;
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<Errors>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::RANGE_NOT_SATISFIABLE,
        ))
    } else if let Some(error) = r.find::<CorsForbidden>() {
        Ok(warp::reply::with_status(
            error.to_string(),
            warp::http::StatusCode::FORBIDDEN,
        ))
    } else if let Some(InvalidID) = r.find() {
        Ok(warp::reply::with_status(
            "no valid ID".to_string(),
            warp::http::StatusCode::UNPROCESSABLE_ENTITY,
        ))
    } else {
        Ok(warp::reply::with_status(
            "Route not found".to_string(),
            warp::http::StatusCode::NOT_FOUND,
        ))
    }
}

async fn home() -> &'static str {
    "Restaurant Api \n\nEndpoints: \n\n/restaurant/id\n/restaurants\n\nUNDER DEVELOPMENT!"
}

async fn create_restaurant() {}

async fn get_single_restaurant() -> Result<impl warp::Reply, warp::Rejection> {
    let d = Restaurant::new(
        RestaurantId(1.to_string()),
        "akbar joje",
        4.8,
        2.8,
        Some(vec!["joje".to_string(), "akbar".to_string()]),
        "img-url",
    );
    match d.id.0.parse::<u32>() {
        Err(_) => Err(warp::reject::custom(InvalidID)),
        Ok(_) => Ok(warp::reply::json(&d)),
    }
}

async fn get_restaurants(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        let res: Vec<Restaurant> = store.restaurants.values().cloned().collect();
        let res = &res[pagination.start..pagination.end];
        Ok(warp::reply::json(&res))
    } else {
        let res: Vec<Restaurant> = store.restaurants.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

// p: 118

// goals:
// - restaurants endpoint return a json of all the restaurants (✅ but its static data)
// - restaurant endpoint accept POST requests and adding the result to restaurants endpoint (❌)
// - restaurant/id returns a json with specific id (❌)

// issues:
// - tests
// - benchmark
// - error handling
