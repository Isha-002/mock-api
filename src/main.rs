use crate::utils::colors::ansi::*;
use routes::authentication_routes::auth_routes;
use routes::comment_routes::comment_routes;
use routes::file_routes::file_routes;
use routes::food_routes::food_routes;
use routes::order_routes::order_routes;
use routes::owner_routes::owner_routes;
use routes::restaurant_routes::restaurant_routes;
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
mod handlers;
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

    // running inital schema
    // you can reset database with --reset
    let _ = &store.init_sql().await;

    if arguments.get_flag("reset") {
        let _ = &store.reset_sql().await;
    }

    if arguments.get_flag("data") {
        let _ = &store.insert_sample_data().await;
    }

    let cors = warp::cors()
        .allow_any_origin()
        .allow_header("content-type")
        .allow_methods([Method::GET, Method::PUT, Method::DELETE, Method::POST]);

    let home = warp::path::end().and(warp::fs::file("static/index.html"));

    let routes = home
        .or(restaurant_routes(store.clone()))
        .or(auth_routes(store.clone()))
        .or(comment_routes(store.clone()))
        .or(food_routes(store.clone()))
        .or(order_routes(store.clone()))
        .or(owner_routes(store.clone()))
        .or(file_routes(store.clone()))
        .with(cors)
        .with(warp::trace::request())
        .recover(return_error);

    println!("{BRIGHT_BLUE}starting the server on http://localhost:4444/{RESET}");
    warp::serve(routes).run(([0, 0, 0, 0], 4444)).await;
}

// 
// 3
// database string 1
// terminal color 9

