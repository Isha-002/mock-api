use warp::Filter;

use crate::{
    handlers::{
        authentication_handlers::auth,
        owner_handlers::{create_owner, get_owner, replace_owner, update_owner_national_id},
    },
    store::Store,
};

pub fn owner_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let get_owner = warp::get()
        .and(warp::path("restaurant"))
        .and(warp::path("owner"))
        .and(auth())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_owner);

    let create_owner = warp::post()
        .and(warp::path("restaurant"))
        .and(warp::path("owner"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(create_owner);

    let update_owner_national_id = warp::put()
        .and(warp::path("restaurant"))
        .and(warp::path("owner"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(update_owner_national_id);

    let replace_owner = warp::put()
        .and(warp::path("restaurant"))
        .and(warp::path("owner"))
        .and(warp::path("replace"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(replace_owner);

        get_owner
        .or(create_owner)
        .or(update_owner_national_id)
        .or(replace_owner)
}
