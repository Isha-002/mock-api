pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    log::info!("rout is called");
    Ok(warp::reply::with_status("Restaurant Api \n\nEndpoints: \n\n/restaurants (get)\n/restaurants (post)\n/restaurants/id (get)\n/restaurants/id (put)\n/restaurants/id (delete)\n\nUNDER DEVELOPMENT!", warp::http::StatusCode::OK))
}
