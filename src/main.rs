
use routes::home::home;
use routes::restaurants::{
    create_restaurant, delete_restaurant, get_restaurants, get_single_restaurant, update_restaurant,
};
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::format;

use types::timer::CustomTimer;
use std::fs::File;
use store::Store;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
mod error;
mod routes;
mod store;
mod types;
use error::return_error;




#[tokio::main]
async fn main() {
    let timer = CustomTimer;

    let file = File::create("log/info.log").expect("couldn't create the log file");
    let (none_blocking, _worker_guard) = tracing_appender::non_blocking(file);

    let log_filter =
        std::env::var("RUST_LOG").unwrap_or_else(|_| "mock-api=info,warp=info".to_owned());
    tracing_subscriber::fmt()
        .with_timer(timer)
        .with_writer(none_blocking)
        .with_ansi(false)
        .with_env_filter(log_filter)
        .with_span_events(FmtSpan::CLOSE)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false).compact().fmt_fields(format::debug_fn(|writer, field, value| {
            write!(writer, "[{}: {:?}]", field, value)
        })

        .delimited(" - "))
        .init();

    let store = Store::init();
    let store_filter = warp::any().map(move || store.clone());

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    let home = warp::get().and(warp::path::end()).and_then(home);

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
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(create_restaurant);

    let get_single_restaurant = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_single_restaurant);

    let update_restaurant = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_restaurant);

    let delete_restaurant = warp::delete()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_restaurant);

    let routes = create_restaurant
        .or(home)
        .or(get_restaurants)
        .or(get_single_restaurant)
        .or(update_restaurant)
        .or(delete_restaurant)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    println!("starting the server on http://localhost:4444/");
    warp::serve(routes).run(([0, 0, 0, 0], 4444)).await;
}

// p: 190

// goals:
// - restaurants endpoint return a json of all the restaurants (✅)
// - restaurants endpoint accept POST requests and adding the result to restaurants endpoint (✅)
// - restaurant/id returns a json with specific id (✅)
// - restaurant/id accept PUT requests and update the restaurant (✅)
// - restaurant/id accept DELETE requests and delete the restaurant (✅)

// issues:
// - tests
// - benchmark (✅)
// - error handling
