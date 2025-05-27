use warp::Filter;

use crate::{
    handlers::authentication_handlers::{login, register},
    store::Store,
};

pub fn auth_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let registration = warp::post()
        .and(warp::path("registration"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(register);

    let login = warp::post()
        .and(warp::path("login"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(login);

    registration.or(login)
}
