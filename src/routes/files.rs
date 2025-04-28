use std::path::PathBuf;

use chrono::Utc;
use uuid::Uuid;
use warp::{filters::multipart::FormData, reject::Rejection, reply::Reply};
use crate::{error::Error, store::Store};
use futures::StreamExt;
use futures::TryStreamExt;
use warp::Buf;

async fn create_upload_dir() -> std::io::Result<()> {
    tokio::fs::create_dir_all("./uploads").await?;
    Ok(())
}

pub async fn upload_file_handler(
  id: i32,
  mut form: FormData,
  store: Store,
) -> Result<impl Reply, Rejection> {
  create_upload_dir()
      .await
      .map_err(|e| warp::reject::custom(Error::creating_upload_dir(e)))?;

      let part = match form.next().await {
        Some(Ok(part)) => part,
        Some(Err(_e)) => return Err(warp::reject::custom(Error::bail_out_card)), 
        None => return Err(warp::reject::custom(Error::no_file)),
    };

  let filename = part
      .filename()
      .map(String::from)
      .unwrap_or_else(|| Uuid::new_v4().to_string());

      let path = PathBuf::from(&filename); 

      let file_ext = path
          .extension()
          .and_then(|ext| ext.to_str())
          .unwrap_or("bin");

  let unique_filename = format!(
      "{}_{}.{}",
      Uuid::new_v4(),
      Utc::now().format("%Y%m%d%H%M%S"),
      file_ext
  );

  let file_path = format!("./uploads/{}", unique_filename);
  let public_url = format!("/files/{}", unique_filename);

  let data = part
      .stream()
      .try_fold(Vec::new(), |mut vec, bytes| async move {
        vec.extend_from_slice(bytes.chunk());
          Ok(vec)
      })
      .await
      .map_err(|_| warp::reject::custom(Error::bail_out_card))?;

  tokio::fs::write(&file_path, &data)
      .await
      .map_err(|e| warp::reject::custom(Error::write_file(e)))?;

  if let Err(e) = store.insert_file_to_restaurant(&public_url, id).await {
      return Err(warp::reject::custom(e));
  }

  Ok(warp::reply::with_status(
      "image added",
      warp::http::StatusCode::OK,
  ))
}

// problem: if there is no restaurant with provided id the image will still be saved in uploads folder. theres no returning error to check if restaurant exists in db. weird i dont know why we get OK in `insert_file_to_restaurant` 
