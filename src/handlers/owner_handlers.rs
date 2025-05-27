use tracing::instrument;

use crate::{
    store::Store,
    types::{
        account::Session,
        owner::{NewOwner, Owner},
    },
};

#[instrument]
pub async fn get_owner(
    session: Session,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let res = match store.get_owner(account_id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn create_owner(
    session: Session,
    owner: NewOwner,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.create_owner(owner, account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "owner registered!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn update_owner_national_id(
    session: Session,
    owner: Owner,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.update_owner_national_id(owner, account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "owner national id updated!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn replace_owner(
    session: Session,
    owner: Owner,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.replace_owner(owner, account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "owner replaced!",
        warp::http::StatusCode::OK,
    ))
}
