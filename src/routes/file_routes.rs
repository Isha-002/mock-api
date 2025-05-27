use warp::Filter;

use crate::{handlers::{authentication_handlers::auth, files_handlers::{food_image_handler, restaurant_pfp_handler}}, store::Store};


pub fn file_routes(
  store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
  let store_filter = warp::any().map(move || store.clone());

    // static routes for serving files
    let files_route = warp::path("upload").and(warp::fs::dir("./uploads"));

    let restaurant_pfp_upload_route = warp::path("restaurants")
        .and(warp::path::param::<i32>())
        .and(warp::path("upload"))
        .and(warp::post())
        .and(warp::multipart::form())
        .and(auth())
        .and(store_filter.clone())
        .and_then(restaurant_pfp_handler);

        let food_image_upload_route = warp::path("restaurants")
        .and(warp::path::param::<i32>())
        .and(warp::path("food"))
        .and(warp::path::param::<i32>())
        .and(warp::path("upload"))
        .and(warp::post())
        .and(warp::multipart::form())
        .and(auth())
        .and(store_filter.clone())
        .and_then(food_image_handler);

        files_route.or(restaurant_pfp_upload_route).or(food_image_upload_route)
}