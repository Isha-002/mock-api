use tracing::{info, instrument};

use crate::{store::Store, types::{account::Session, restaurant::OpenHours}};

#[instrument]
pub async fn get_restaurant_hours(
    id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.get_restaurant_hours(id).await {
        Ok(hours) => hours,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn post_restaurant_hours(
    session: Session,
    hours: OpenHours,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.post_restaurant_hours(account_id, hours).await {
        return Err(warp::reject::custom(e));
    }
    info!("hours added");
    Ok(warp::reply::with_status(
        "hours added!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn put_restaurant_hours(
    session: Session,
    hours: OpenHours,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let res = match store.put_restaurant_hours(account_id, hours).await {
        Ok(hours) => hours,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    info!("hours updated");
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn delete_restaurant_hours(
    session: Session,
    hours: OpenHours,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if let Err(e) = store.delete_restaurant_hours(account_id, hours).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "food deleted",
        warp::http::StatusCode::OK,
    ))
}
