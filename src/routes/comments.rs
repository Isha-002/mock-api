use tracing::{info, instrument};

use crate::{store::Store, types::comment::NewComment};

#[instrument]
pub async fn get_comments(
    restaurant_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    let res = match store.get_comments(restaurant_id).await {
        Ok(res) => res,
        Err(e) => return Err(warp::reject::custom(e)),
    };
    Ok(warp::reply::json(&res))
}

#[instrument]
pub async fn put_comments(
    restaurant_id: i32,
    store: Store,
    comment: NewComment,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_comment(restaurant_id, comment).await {
        return Err(warp::reject::custom(e));
    }
    info!("comment added");
    Ok(warp::reply::with_status(
        "comment added!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn add_like_comment(
    restaurant_id: i32,
    comment_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_comment_like(restaurant_id, comment_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "liked!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn add_dislike_comment(
    restaurant_id: i32,
    comment_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.add_comment_dislike(restaurant_id, comment_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "liked!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn delete_like_comment(
    restaurant_id: i32,
    comment_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store.remove_comment_like(restaurant_id, comment_id).await {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "liked!",
        warp::http::StatusCode::OK,
    ))
}

#[instrument]
pub async fn delete_dislike_comment(
    restaurant_id: i32,
    comment_id: i32,
    store: Store,
) -> Result<impl warp::Reply, warp::Rejection> {
    if let Err(e) = store
        .remove_comment_dislike(restaurant_id, comment_id)
        .await
    {
        return Err(warp::reject::custom(e));
    }
    Ok(warp::reply::with_status(
        "liked!",
        warp::http::StatusCode::OK,
    ))
}
