use warp::Filter;

use crate::{
    handlers::{
        authentication_handlers::auth,
        comment_handlers::{comment_vote, delete_comment, get_comments, post_comments},
    },
    store::Store,
};

pub fn comment_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let get_comments = warp::get()
        .and(warp::path("restaurants"))
        .and(auth())
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_comments);

    let post_comments = warp::post()
        .and(warp::path("restaurants"))
        .and(auth())
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(post_comments);

    let delete_comment = warp::delete()
        .and(warp::path("restaurants"))
        .and(auth())
        .and(warp::path("comments"))
        .and(warp::path::param::<i32>())
        .and(warp::path("delete"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_comment);

    let comment_vote = warp::post()
        .and(warp::path("restaurants"))
        .and(auth())
        .and(warp::path("comments"))
        .and(warp::path("vote"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(comment_vote);

    get_comments
        .or(post_comments)
        .or(delete_comment)
        .or(comment_vote)
}
