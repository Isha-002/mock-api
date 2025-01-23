use std::collections::HashMap;

use tracing::{error, event, info, instrument, Level};

use crate::{
    error::{Error, InvalidID},
    store::Store,
    types::{
        pagination::extract_pagination,
        restaurant::{Restaurant, RestaurantId},
    },
};

#[instrument]
pub async fn create_restaurant(
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    store
        .restaurants
        .write()
        .await
        .insert(restaurant.id.clone(), restaurant);
    info!("restaurant added");
    Ok(warp::reply::with_status(
        "restaurant added!",
        warp::http::StatusCode::OK,
    ))
}
#[instrument]
pub async fn get_single_restaurant(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let restaurant = store.restaurants.read().await;
    match restaurant.get(&RestaurantId(id)) {
        Some(res) => {
            info!("querying one restaurant");
            Ok(warp::reply::json(res))
        }
        None => {
            if id.is_negative() {
                error!("restaurant id invalid");
                Err(warp::reject::custom(InvalidID))
            } else if !restaurant.contains_key(&RestaurantId(id)) {
                error!("restaurant id not found");
                Err(warp::reject::custom(Error::restaurant_not_found))
            } else {
                error!("restaurant not found unknown error");
                Err(warp::reject::custom(Error::unkown_error))
            }
        }
    }
}
#[instrument]
pub async fn get_restaurants(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(Level::INFO, "querying restaurants");
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        if pagination.start == 0
            || pagination.end > store.restaurants.read().await.len()
            || pagination.start > pagination.end
        {
            event!(Level::ERROR, pagination = false);
            Err(warp::reject::custom(Error::unacceptable_parameters))
        } else {
            event!(Level::INFO, pagination = true);
            let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
            let res = &res[pagination.start - 1..pagination.end];
            Ok(warp::reply::json(&res))
        }
    } else {
        event!(Level::INFO, pagination = false);
        let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}
#[instrument]
pub async fn update_restaurant(
    id: i32,
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match store.restaurants.write().await.get_mut(&RestaurantId(id)) {
        Some(r) => {
            *r = restaurant;
            Ok::<(), Error>(())
        }
        None => {
            return {
                error!("restaurant not found");
                Err(warp::reject::custom(Error::restaurant_not_found))
            }
        }
    };
    info!("restaurant updated");
    Ok(warp::reply::with_status(
        "restaurant modified",
        warp::http::StatusCode::OK,
    ))
}
#[instrument]
pub async fn delete_restaurant(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.restaurants.write().await.remove(&RestaurantId(id)) {
        Some(_) => {
            info!("restaurant deleted");
            Ok(warp::reply::with_status(
                "restaurant deleted",
                warp::http::StatusCode::OK,
            ))
        }
        None => {
            error!("restaurant not found");
            Err(warp::reject::custom(Error::restaurant_not_found))
        }
    }
}
