use warp::filters::multipart::FormData;
use warp::reject::Rejection;
use std::path::PathBuf;

use crate::error::Error;
use chrono::Utc;
use futures::StreamExt;
use futures::TryStreamExt;
use uuid::Uuid;
use warp::Buf;

async fn create_upload_dir() -> std::io::Result<()> {
    tokio::fs::create_dir_all("./uploads").await?;
    Ok(())
}

pub async fn upload_file(file_type: String, mut form: FormData) -> Result<String, Rejection> {
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
        "{}_{}_{}.{}",
        file_type,
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
    
    Ok(public_url)
}
