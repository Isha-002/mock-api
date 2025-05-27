use std::collections::HashMap;

use tracing::{info, instrument};

use crate::{store::Store, types::{account::Session, comment::NewComment, pagination::extract_params}};

#[instrument]
pub async fn get_comments(
    session: Session,
    restaurant_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    let res = match store.get_comments(restaurant_id, account_id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn post_comments(
    session: Session,
    restaurant_id: i32,
    store: Store,
    comment: NewComment,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.add_comment(restaurant_id, comment, account_id).await {
        return Err(warp::reject::custom(e));
    }
    info!("comment added");
    Ok(warp::reply::with_status(
        "comment added!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn delete_comment(
    session: Session,
    comment_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let account_id = session.account_id;
    if let Err(e) = store.delete_comment(comment_id, account_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "Deleted",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn comment_vote(
    session: Session,
    params: HashMap<String, String>,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {

    let comment_id = extract_params::<i32>(&params, "id")?;
    let vote = extract_params::<i32>(&params, "vote")?;
    let account_id = session.account_id;

    if let Err(e) = store.comment_vote(account_id, comment_id, vote).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "liked!",
        warp::http::StatusCode::OK,
    ))
}

