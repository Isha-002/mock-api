use tracing::instrument;

use crate::{
    store::Store,
    types::{account::Session, orders::NewItem},
};

#[instrument]
pub async fn get_customer_orders(
    session: Session,
    order_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let res = match store.get_customer_orders(account_id, order_id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn create_cart(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.create_cart(account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "cart created!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn add_to_cart(
    session: Session,
    item: NewItem,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.add_to_cart(account_id, item).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "cart updated!",
        warp::http::StatusCode::OK,
    ))
}
