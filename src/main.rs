use core::option::Option::Some;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::{
    fmt::{self},
    io::{Error, ErrorKind},
    str::FromStr,
};
use tokio::sync::RwLock;
use warp::{
    filters::{body::BodyDeserializeError, cors::CorsForbidden},
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
    unacceptable_parameters,
    restaurant_not_found,
    unkown_error,
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
            Errors::unacceptable_parameters => {
                write!(f, "parameters are not acceptable")
            }
            Errors::restaurant_not_found => {
                write!(f, "restaurant not found")
            }
            Errors::unkown_error => {
                write!(f, "something happened and we dont know what!")
            }
        }
    }
}

impl Reject for Errors {}

#[derive(Clone, Debug)]
struct Store {
    restaurants: Arc<RwLock<HashMap<RestaurantId, Restaurant>>>,
}

impl Store {
    fn init() -> Self {
        let mock_data = include_str!("../mock_data.json");
        match serde_json::from_str(mock_data) {
            Ok(data) => Store {
                restaurants: Arc::new(RwLock::new(data)),
            },
            Err(e) => {
                println!("there was an error reading mock_data.json: {e}");
                println!("initialized Empty...");
                Store::new()
            }
        }
    }
    fn new() -> Self {
        Store {
            restaurants: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    async fn is_empty(&self) -> bool {
        self.restaurants.read().await.is_empty()
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
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    let home = warp::get().and(warp::path::end()).and_then(home);

    let get_restaurants = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_restaurants);

    let create_restaurant = warp::post()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(create_restaurant);

    let get_single_restaurant = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_single_restaurant);

    let update_restaurant = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_restaurant);

    let delete_restaurant = warp::delete()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_restaurant);

    let routes = create_restaurant
        .or(home)
        .or(get_restaurants)
        .or(get_single_restaurant)
        .or(update_restaurant)
        .or(delete_restaurant)
        .with(cors)
        .recover(return_error);

    println!("starting the server on http://localhost:4444/");
    warp::serve(routes).run(([0, 0, 0, 0], 4444)).await;
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

async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    Ok(warp::reply::with_status("Restaurant Api \n\nEndpoints: \n\n/restaurants (get)\n/restaurants (post)\n/restaurants/id (get)\n\nUNDER DEVELOPMENT!", warp::http::StatusCode::OK))
}

async fn create_restaurant(
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .restaurants
        .write()
        .await
        .insert(restaurant.id.clone(), restaurant);
    Ok(warp::reply::with_status(
        "restaurant added!",
        warp::http::StatusCode::OK,
    ))
}

async fn get_single_restaurant(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let restaurant = store.restaurants.read().await;
    match restaurant.get(&RestaurantId(id.clone())) {
        Some(res) => Ok(warp::reply::json(res)),
        None => {
            if id.parse::<u32>().is_err() {
                Err(warp::reject::custom(InvalidID))
            } else if !restaurant.contains_key(&RestaurantId(id)) {
                Err(warp::reject::custom(Errors::restaurant_not_found))
            } else {
                Err(warp::reject::custom(Errors::unkown_error))
            }
        }
    }
}

async fn get_restaurants(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        if pagination.start == 0
            || pagination.end > store.restaurants.read().await.len()
            || pagination.start > pagination.end
        {
            Err(warp::reject::custom(Errors::unacceptable_parameters))
        } else {
            let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
            let res = &res[pagination.start - 1..pagination.end];
            Ok(warp::reply::json(&res))
        }
    } else {
        let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

async fn update_restaurant(
    id: String,
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match store.restaurants.write().await.get_mut(&RestaurantId(id)) {
        Some(r) => {
            *r = restaurant;
            Ok::<(), Errors>(())
        }
        None => return Err(warp::reject::custom(Errors::restaurant_not_found)),
    };

    Ok(warp::reply::with_status(
        "restaurant modified",
        warp::http::StatusCode::OK,
    ))
}

async fn delete_restaurant(id: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    match store.restaurants.write().await.remove(&RestaurantId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "restaurant deleted",
            warp::http::StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Errors::restaurant_not_found)),
    }
}

// p: 127

// goals:
// - restaurants endpoint return a json of all the restaurants (✅ but its static data)
// - restaurant endpoint accept POST requests and adding the result to restaurants endpoint (✅)
// - restaurant/id returns a json with specific id (✅)
// - restaurant/id accept PUT requests and update the restaurant (❌)
// - restaurant/id accept DELETE requests and delete the restaurant (❌)

// issues:
// - tests
// - benchmark (✅)
// - error handling

// as: 95
//
