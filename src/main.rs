// #![recursion_limit = "256"]
use routes::authentication::{login, register};
use routes::comments::{
    add_dislike_comment, add_like_comment, delete_dislike_comment, delete_like_comment,
    get_comments, put_comments,
};
use routes::files::restaurant_pfp_handler;
use routes::restaurants::{
    delete_restaurant, get_restaurants, get_single_restaurant, search_by_city, search_by_tag,
    update_restaurant,
};
use routes::{home::home, restaurants::create_restaurant};
use tracing_subscriber::field::MakeExt;
use tracing_subscriber::fmt::format;
use utils::arguments::arguments;

use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::panic;
use store::Store;
use types::timer::CustomTimer;

use tracing_subscriber::fmt::format::FmtSpan;
use warp::{http::Method, Filter};
mod error;
mod routes;
mod store;
mod types;
mod utils;
use error::return_error;

#[tokio::main]
async fn main() {
    panic::set_hook(Box::new(|info| {
        let mut file = File::create("restaurant_api_error_log.txt").unwrap();
        let _ = writeln!(file, "Panic occurred: {}", info);
    }));

    let arguments = arguments();

    let timer = CustomTimer;

    create_dir_all("log").expect("failed to create log directory");
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("log/info.log")
        .expect("couldn't open or create the log file");
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
        .with_thread_names(false)
        .compact()
        .fmt_fields(
            format::debug_fn(|writer, field, value| write!(writer, "[{}: {:?}]", field, value))
                .delimited(" - "),
        )
        .init();

    let store = Store::new("postgres://postgres:4431@localhost:5432/restaurantapi").await;
    let store_for_filters = store.clone();
    let store_filter = warp::any().map(move || store_for_filters.clone());

    if arguments.get_flag("reset") {
        let _ = &store.migrate().await;
    }
    if arguments.get_flag("data") {
        let _ = &store.insert_fake_data().await;
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    let home  = warp::path::end().and(warp::fs::file("static/index.html"));
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
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_single_restaurant);

    let update_restaurant = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(update_restaurant);

    let delete_restaurant = warp::delete()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_restaurant);

    let get_comments = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(get_comments);

    let put_comments = warp::post()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and(warp::body::json())
        .and_then(put_comments);

    let add_comments_like = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::param::<i32>())
        .and(warp::path("likes"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(add_like_comment);

    let add_comments_dislike = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::param::<i32>())
        .and(warp::path("dislikes"))
        .and(warp::path("add"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(add_dislike_comment);

    let delete_comments_like = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::param::<i32>())
        .and(warp::path("likes"))
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_like_comment);

    let delete_comments_dislike = warp::put()
        .and(warp::path("restaurants"))
        .and(warp::path::param::<i32>())
        .and(warp::path("comments"))
        .and(warp::path::param::<i32>())
        .and(warp::path("dislike"))
        .and(warp::path("remove"))
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(delete_dislike_comment);

    // search
    let search_by_city = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path("city"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(search_by_city);

    let search_by_tag = warp::get()
        .and(warp::path("restaurants"))
        .and(warp::path("tag"))
        .and(warp::path::param::<String>())
        .and(warp::path::end())
        .and(store_filter.clone())
        .and_then(search_by_tag);

    // static routes for serving files
    let files_route = warp::path("upload").and(warp::fs::dir("./uploads"));

    let restaurant_pfp_upload_route = warp::path("restaurants")
        .and(warp::path::param::<i32>())
        .and(warp::path("upload"))
        .and(warp::post())
        .and(warp::multipart::form())
        .and(store_filter.clone())
        .and_then(restaurant_pfp_handler);

    // let get_file_route = warp::path("files")
    //     .and(warp::path::param::<i32>())
    //     .and(warp::get())
    //     .and(store_filter.clone())
    //     .and_then(get_file_handler);

    // auth routes
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

    // 
    let static_files = warp::fs::dir("static");

    let routes = create_restaurant
        .or(home)
        .or(get_restaurants)
        .or(get_single_restaurant)
        .or(update_restaurant)
        .or(delete_restaurant)
        .or(get_comments)
        .or(put_comments)
        .or(add_comments_like)
        .or(add_comments_dislike)
        .or(delete_comments_like)
        .or(delete_comments_dislike)
        .or(search_by_city)
        .or(search_by_tag)
        .or(files_route)
        .or(restaurant_pfp_upload_route)
        .or(registration)
        .or(login)
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    println!("starting the server on http://localhost:4444/");
    warp::serve(routes).run(([0, 0, 0, 0], 4444)).await;
}



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
