use warp::Filter;

use crate::{handlers::{authentication_handlers::auth, food_handlers::{delete_food, get_menu, post_new_food, update_food}}, store::Store};

pub fn food_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let get_menu = warp::get()
    .and(warp::path("restaurants"))
    .and(warp::path::param::<i32>())
    .and(warp::path("food"))
    .and(warp::path::end())
    .and(store_filter.clone())
    .and_then(get_menu);


    let post_new_food = warp::post()
    .and(warp::path("restaurants"))
    .and(auth())
    .and(warp::path("food"))
    .and(warp::path::end())
    .and(warp::body::json())
    .and(store_filter.clone())
    .and_then(post_new_food);


    let update_food = warp::put()
    .and(warp::path("restaurants"))
    .and(auth())
    .and(warp::path("food"))
    .and(warp::path::end())
    .and(warp::body::json())
    .and(store_filter.clone())
    .and_then(update_food);


    let delete_food = warp::delete()
    .and(warp::path("restaurants"))
    .and(auth())
    .and(warp::path("food"))
    .and(warp::path::end())
    .and(warp::body::json())
    .and(store_filter.clone())
    .and_then(delete_food);


    get_menu
        .or(post_new_food)
        .or(update_food)
        .or(delete_food)
}
