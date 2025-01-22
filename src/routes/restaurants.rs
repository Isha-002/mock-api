use std::collections::HashMap;

use tracing::{info, instrument};

use crate::{
    error::{Error, InvalidID},
    store::Store,
    types::{
        pagination::extract_pagination,
        restaurant::{Restaurant, RestaurantId},
    },
};

pub async fn create_restaurant(
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

pub async fn get_single_restaurant(
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
                Err(warp::reject::custom(Error::restaurant_not_found))
            } else {
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
    info!("querying questions");
    if !params.is_empty() {
        let pagination = extract_pagination(params)?;
        if pagination.start == 0
            || pagination.end > store.restaurants.read().await.len()
            || pagination.start > pagination.end
        {
            info!(pagination = false);
            Err(warp::reject::custom(Error::unacceptable_parameters))
        } else {
            info!(pagination = true);
            let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
            let res = &res[pagination.start - 1..pagination.end];
            Ok(warp::reply::json(&res))
        }
    } else {
        info!(pagination = false);
        let res: Vec<Restaurant> = store.restaurants.read().await.values().cloned().collect();
        Ok(warp::reply::json(&res))
    }
}

pub async fn update_restaurant(
    id: String,
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let _ = match store.restaurants.write().await.get_mut(&RestaurantId(id)) {
        Some(r) => {
            *r = restaurant;
            Ok::<(), Error>(())
        }
        None => return Err(warp::reject::custom(Error::restaurant_not_found)),
    };

    Ok(warp::reply::with_status(
        "restaurant modified",
        warp::http::StatusCode::OK,
    ))
}

pub async fn delete_restaurant(
    id: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    match store.restaurants.write().await.remove(&RestaurantId(id)) {
        Some(_) => Ok(warp::reply::with_status(
            "restaurant deleted",
            warp::http::StatusCode::OK,
        )),
        None => Err(warp::reject::custom(Error::restaurant_not_found)),
    }
}
