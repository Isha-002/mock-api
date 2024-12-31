use core::option::Option::Some;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{
    fmt::{self},
    io::{Error, ErrorKind},
    str::FromStr,
};
use warp::{
    filters::cors::CorsForbidden,
    http::Method,
    reject::{Reject, Rejection},
    reply::Reply,
    Filter,
};

struct Store {
    restaurants: HashMap<RestaurantId, Restaurant>,
}

impl Store {
    fn init(mut self) -> Self {
        // this method will only work if your data structure is empty!!
        // only use it if you want to initialize with mock data!!
        let data = [
            Restaurant::new(
                RestaurantId(1.to_string()), 
                "Akbar Joojeh", 
                3.2, 
                5.3, 
                Some(vec![
                    "akbar jooje".to_string(), 
                    "Sos".to_string(), 
                    "Berenj".to_string()
                ]), 
                "https://www.akbarjoojeh.com/wp-content/uploads/2020/02/IMG_20200205_104013_404-1024x768.jpg"
            ),
            Restaurant::new(
                RestaurantId(2.to_string()), 
                "Bodega", 
                4.5, 
                8.1, 
                Some(vec![
                    "Tacos".to_string(), 
                    "Burritos".to_string(), 
                    "Quesadillas".to_string()
                ]), 
                "https://www.lettersandsigns.com/wp-content/uploads/elementor/thumbs/Layered-Red-Black-Acrylic-Sign-Bodega-Restaurant-1-pzb6prcdrc5le6apypbx8u2667vq157h06y9m90q06.jpg"
            ),
            Restaurant::new(
                RestaurantId(3.to_string()), 
                "Maggie", 
                2.8, 
                3.2, 
                Some(vec![
                    "Pizza".to_string(), 
                    "Pasta".to_string(), 
                    "Salads".to_string()
                ]), 
                "https://www.lettersandsigns.com/wp-content/uploads/2020/06/custom-formed-plastic-letters-edge-paint-maggie.jpg"
            ),
        ];

        if self.is_empty() {
            for d in data {
            self.restaurants.insert(d.id.clone(), d);
        };
        }
        self
        
    }
    fn new() -> Self {
        Store {
            restaurants: HashMap::new(),
        }
    }
    fn add_restaurant(mut self, r: Restaurant) -> Self {
        self.restaurants.insert(r.id.clone(), r);
        self
    }
    fn is_empty(&self) -> bool {
        self.restaurants.is_empty()
    }
}

#[derive(Debug)]
struct InvalidID;
impl Reject for InvalidID {}

#[derive(Serialize, Deserialize)]
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
    let _data: Vec<Restaurant> = vec![];
    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    // let home = warp::path("/").map(|| "home".to_string());
    let res = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and_then(get_restaurants)
        .recover(return_error);
    warp::serve(res.with(cors)).run(([0, 0, 0, 0], 4444)).await;
}

async fn return_error(r: Rejection) -> Result<impl Reply, Rejection> {
    println!("{:?}", r);
    if let Some(error) = r.find::<CorsForbidden>() {
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

async fn get_restaurants() -> Result<impl warp::Reply, warp::Rejection> {
    let data = vec![
        Restaurant::new(
            RestaurantId(1.to_string()),
            "akbar joje",
            4.8,
            2.8,
            Some(vec!["joje".to_string(), "akbar".to_string()]),
            "img-url",
        ),
        Restaurant::new(
            RestaurantId(2.to_string()),
            "akbar not joje",
            4.7,
            2.8,
            Some(vec!["no joje".to_string(), "big akbar".to_string()]),
            "img-url",
        ),
        Restaurant::new(
            RestaurantId(3.to_string()),
            "akbar very joje",
            2.2,
            2.8,
            Some(vec!["very joje".to_string(), "very akbar".to_string()]),
            "img-url",
        ),
    ];
    Ok(warp::reply::json(&data))
}

// p: 107

// goals:
// - restaurants endpoint return a json of all the restaurants (✅ but its static data)
// - restaurant endpoint accept POST requests and adding the result to restaurants endpoint (❌)
// - restaurant/id returns a json with specific id (❌)

// issues:
// - tests
// - benchmark
