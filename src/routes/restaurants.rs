use std::collections::HashMap;

use tracing::{event, info, instrument, Level};

use crate::{
    store::Store,
    types::{
        pagination::{extract_pagination, Pagination},
        restaurant::{ NewRestaurant, Restaurant},
    },
};

#[instrument]
pub async fn create_restaurant(
    store: Store,
    restaurant: NewRestaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_restaurant(restaurant).await {
        return Err(warp::reject::custom(e));
    }
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
    let res = match store.get_single_restaurant(id).await {
        Ok(restaurant) => restaurant,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn get_restaurants(
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    event!(target: "restaurant_api", Level::INFO, "querying restaurants");
    let mut pagination = Pagination::default();
    if !params.is_empty() {
        event!(Level::INFO, pagination = true);
        pagination = extract_pagination(params)?;

        let res = match store
            .get_restaurants(pagination.limit, pagination.offset)
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        Ok(warp::reply::json(&res))
    } else {
        event!(Level::INFO, pagination = false);
        let res = match store
            .get_restaurants(pagination.limit, pagination.offset)
            .await
        {
            Ok(res) => res,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        Ok(warp::reply::json(&res))
    }
}

#[instrument]
pub async fn update_restaurant(
    id: i32,
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.update_restaurant(restaurant, id).await {
        Ok(restaurant) => restaurant,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    info!("restaurant updated");
    Ok(warp::reply::json(&res))
}
#[instrument]
pub async fn delete_restaurant(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.delete_restaurant(id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        format!("Restaurant {} deleted", id),
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn search_by_city(city: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.search_by_city(city).await {
        Ok(restaurants) => restaurants,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn search_by_tag(tag: String, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.search_by_tag(tag).await {
        Ok(restaurants) => restaurants,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}