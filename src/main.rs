use axum::Json;
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
    let data = Restaurant::new(
        RestaurantId(1.to_string()),
        "sigma",
        2.2,
        2.8,
        Some(vec!["hi".to_string()]),
        "cat pic",
    );

    let app = Router::new()
        .route("/", get(home))
        .route("/restaurants", get(create_restaurant));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4444").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn home() -> &'static str {
    "Restaurant Api \n\nEndpoints: \n\n/restaurant/id\n/restaurants\n\nUNDER DEVELOPMENT!"
}

async fn create_restaurant() -> impl IntoResponse {
    let data = Restaurant::new(
        RestaurantId(1.to_string()),
        "sigma",
        2.2,
        2.8,
        Some(vec!["hi".to_string()]),
        "cat pic",
    );
    let json = to_string_pretty(&data).unwrap();
    (axum::http::StatusCode::OK, json).into_response()
}

// async fn restaurants() -> String {

// }
// 52