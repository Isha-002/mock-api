use warp::Filter;

use crate::{
    handlers::{
        authentication_handlers::auth,
        order_handlers::{add_to_cart, create_cart, get_customer_orders},
    },
    store::Store,
};

pub fn order_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());

    let get_customer_orders = warp::get()
        .and(warp::path("order"))
        .and(auth())
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_customer_orders);

    let create_cart = warp::post()
        .and(warp::path("order"))
        .and(auth())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(create_cart);

    let add_to_cart = warp::put()
        .and(warp::path("order"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(add_to_cart);

    get_customer_orders.or(create_cart).or(add_to_cart)
}
