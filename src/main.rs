use axum::extract::Path;
use axum::routing::post;
use axum::{response::IntoResponse, routing::get, Router};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty;
use std::{
    fmt::{self},
    io::{Error, ErrorKind},
    str::FromStr,
};

#[derive(Serialize, Deserialize)]
struct Restaurant {
    id: RestaurantId,
    name: String,
    rating: f32,
    distance: f64,
    tags: Option<Vec<String>>,
    image: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
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

    let app = Router::new()
        .route("/", get(home))
        .route("/restaurants", get(get_restaurants))
        .route("/restaurant", post(create_restaurant))
        .route("/restaurant/:id", get(get_restaurant));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await.unwrap();
    println!("Listening on: http://localhost:4444/");
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Restaurant Api \n\nEndpoints: \n\n/restaurant/id\n/restaurants\n\nUNDER DEVELOPMENT!"
}

async fn create_restaurant() {

}

async fn get_restaurants() -> impl IntoResponse {
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

    let json = to_string_pretty(&data).unwrap();
    (axum::http::StatusCode::OK, json).into_response()
}

async fn get_restaurant(Path(id): Path<String>) -> impl IntoResponse {
    format!("restaurant id: {id}")
}

// page: 66

// goals

// restaurants endpoint return a json of all the restaurants (✅ but its static data)
// restaurant endpoint accept POST requests and adding the result to restaurants endpoint (❌)
// restaurant/id returns a json with specific id (❌)

// issues
// tests dont work
