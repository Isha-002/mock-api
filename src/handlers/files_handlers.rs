use warp::{filters::multipart::FormData, reject::Rejection, reply::Reply};

use crate::{error::Error, store::Store, types::account::Session, utils::upload_file::upload_file};

pub async fn restaurant_pfp_handler(
    id: i32,
    form: FormData,
    session: Session,
    store: Store,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    if store
        .verify_restaurant_modification_access(id, &account_id)
        .await?
    {
        let public_url = match upload_file("restaurant_pfp".to_string(), form).await {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        if let Err(e) = store.insert_file_to_restaurant(&public_url, id).await {
            return Err(warp::reject::custom(e));
        }

        Ok(warp::reply::with_status(
            "image added",
            warp::http::StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(Error::not_authorized))
    }
}

// problem: if there is no restaurant with provided id the image will still be saved in uploads folder. theres no returning error to check if restaurant exists in db. weird i dont know why we get OK in `insert_file_to_restaurant`


pub async fn food_image_handler(
    id: i32,
    food_id: i32,
    form: FormData,
    session: Session,
    store: Store,
) -> Result<impl Reply, Rejection> {
    let account_id = session.account_id;
    if store
        .verify_restaurant_modification_access(id, &account_id)
        .await?
    {
        let public_url = match upload_file("food_img".to_string(), form).await {
            Ok(url) => url,
            Err(e) => return Err(e),
        };
        if let Err(e) = store.insert_food_image(&public_url, food_id, id).await {
            return Err(warp::reject::custom(e));
        }

        Ok(warp::reply::with_status(
            "image added",
            warp::http::StatusCode::OK,
        ))
    } else {
        Err(warp::reject::custom(Error::not_authorized))
    }
}