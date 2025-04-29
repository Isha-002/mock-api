
use warp::{filters::multipart::FormData, reject::Rejection, reply::Reply};

use crate::{store::Store, utils::upload_file::upload_file};

pub async fn restaurant_pfp_handler(
    id: i32,
    form: FormData,
    store: Store,
) -> Result<impl Reply, Rejection> {
    
    let public_url = match upload_file("restaurant_pfp".to_string(), form).await {
        Ok(url) => url,
        Err(e) => return Err(e)
    };
    if let Err(e) = store.insert_file_to_restaurant(&public_url, id).await {
        return Err(warp::reject::custom(e));
    }

    Ok(warp::reply::with_status(
        "image added",
        warp::http::StatusCode::OK,
    ))
}

// problem: if there is no restaurant with provided id the image will still be saved in uploads folder. theres no returning error to check if restaurant exists in db. weird i dont know why we get OK in `insert_file_to_restaurant`
