use tracing::{info, instrument};

use crate::{
    store::Store,
    types::{
        account::Session,
        food::{Food, NewFood},
    },
};

#[instrument]
pub async fn get_menu(id: i32, store: Store) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.get_menu(id).await {
        Ok(food) => food,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn post_new_food(
    session: Session,
    food: NewFood,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.post_new_food(food, account_id).await {
        return Err(warp::reject::custom(e));
    }
    info!("food added");
    Ok(warp::reply::with_status(
        "food added!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn update_food(
    session: Session,
    food: Food,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let res = match store.update_food(food, account_id).await {
        Ok(food) => food,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    info!("food updated");
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn delete_food(
    session: Session,
    food: Food,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;

    if let Err(e) = store.delete_food(food, account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "food deleted",
        warp::http::StatusCode::OK,
    ))
}
