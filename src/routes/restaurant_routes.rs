use crate::handlers::authentication_handlers::auth;
use crate::handlers::hours_handler::{
    delete_restaurant_hours, get_restaurant_hours, post_restaurant_hours, put_restaurant_hours,
};
use crate::handlers::restaurant_handlers::{
    create_restaurant, delete_restaurant, get_restaurants, get_single_restaurant, search_by_city,
    search_by_tag, update_restaurant,
};
use crate::store::Store;
use warp::Filter;

pub fn restaurant_routes(
    store: Store,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let store_filter = warp::any().map(move || store.clone());
    
    // restaurant ///////////////////////////////////////////////////////////////////////////////
    let get_restaurants = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(warp::query())
        .and(store_filter.clone())
        .and_then(get_restaurants)
        .with(warp::trace(|info| {
            tracing::info_span!(
                "get restaurants request",
                method = %info.method(),
                path = %info.path(),
                id = %uuid::Uuid::new_v4(),
            )
        }));

    let create_restaurant = warp::post()
        .and(warp::path("restaurants"))
        .and(warp::path::end())
        .and(auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(create_restaurant);

    let get_single_restaurant = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_single_restaurant);

    let update_restaurant = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_restaurant);

    let delete_restaurant = warp::delete()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(auth())
        .and(store_filter.clone())
        .and_then(delete_restaurant);
    // search ///////////////////////////////////////////////////////////////////////////////
    let search_by_city = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path("city"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(search_by_city);

    let search_by_tag = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::query())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(search_by_tag);

    // hours ///////////////////////////////////////////////////////////////////////////////
    let get_restaurant_hours = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path("hours"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_restaurant_hours);

    let post_restaurant_hours = warp::post()
        .and(warp::path("restaurants"))
        .and(warp::path("hours"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(post_restaurant_hours);

    let put_restaurant_hours = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path("hours"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(put_restaurant_hours);

    let delete_restaurant_hours = warp::delete()
        .and(warp::path("restaurants"))
        .and(warp::path("hours"))
        .and(auth())
        .and(warp::path::end())
        .and(warp::body::json())
        .and(store_filter.clone())
        .and_then(delete_restaurant_hours);
    /////////////////////////////////////////////////////////////////////////////////////

    get_restaurants
        // restaurant
        .or(create_restaurant)
        .or(get_single_restaurant)
        .or(update_restaurant)
        .or(delete_restaurant)
        // search
        .or(search_by_city)
        .or(search_by_tag)
        // hours
        .or(get_restaurant_hours)
        .or(post_restaurant_hours)
        .or(put_restaurant_hours)
        .or(delete_restaurant_hours)
}
