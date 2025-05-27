use std::collections::HashMap;

use tracing::{event, info, instrument, Level};

use crate::{
    error::Error,
    store::Store,
    types::{
        account::{Role, Session},
        pagination::{extract_pagination, extract_params, Pagination},
        restaurant::{NewRestaurant, Restaurant},
    },
    verify_roles,
};

#[instrument]
pub async fn create_restaurant(
    session: Session,
    store: Store,
    restaurant: NewRestaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    match verify_roles!(store, &account_id, Role::admin, Role::restaurant_owner).await {
        Ok(true) => {
            if let Err(e) = store.add_restaurant(restaurant).await {
                return Err(warp::reject::custom(e));
            }
            info!("restaurant added");
            Ok(warp::reply::with_status(
                "restaurant added!",
                warp::http::StatusCode::OK,
            ))
        }
        Ok(false) => Err(warp::reject::custom(Error::not_authorized)),
        Err(e) => Err(warp::reject::custom::<Error>(e)),
    }
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
    session: Session,
    store: Store,
    restaurant: Restaurant,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if store
        .verify_restaurant_modification_access(id, &account_id)
        .await?
    {
        let res = match store.update_restaurant(restaurant, id).await {
            Ok(restaurant) => restaurant,
            Err(e) => return Err(warp::reject::custom(e)),
        };
        info!("restaurant updated");
        Ok(warp::reply::json(&res))
    } else {
        Err(warp::reject::custom(Error::not_authorized))
    }
}

#[instrument]
pub async fn delete_restaurant(
    id: i32,
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if store
        .verify_restaurant_modification_access(id, &account_id)
        .await?
    {
        if let Err(e) = store.delete_restaurant(id).await {
            return Err(warp::reject::custom(e));
        }
        Ok(warp::reply::with_status(
            format!("Restaurant {} deleted", id),
            warp::http::StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(Error::not_authorized))
    }
}

#[instrument]
pub async fn search_by_city(
    city: String,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.search_by_city(city).await {
        Ok(restaurants) => restaurants,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn search_by_tag(params: HashMap<String, String>, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    // check out for these guys
    // TEST: error propagations
    // TEST: persian input
    let tag = extract_params::<String>(&params, "tag")?;
    let city = extract_params::<String>(&params, "city")?;
    let res = match store.search_by_tag(tag, city).await {
        Ok(restaurants) => restaurants,
        Err(e) => return Err(warp::reject::custom(e)),
    };

    Ok(warp::reply::json(&res))
}
