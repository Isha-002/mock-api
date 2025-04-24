pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let response = format!("{ASCII}
    \n\nRestaurant Api \n\nEndpoints: \n\n/restaurants (get)\n/restaurants (post)\n/restaurants/id (get)\n/restaurants/id (put)\n/restaurants/id (delete)\n\nUNDER DEVELOPMENT!");
    Ok(warp::reply::with_status(response, warp::http::StatusCode::OK))
}

const ASCII: &str = r#"
                  ___          /|
|||| ||||     .-"`   `"-.     } |  __
|||| ||||   .'  .-'`'-.  '.   } | /  \
|||| \  /  /  .'       '.  \  } | ;();
\  /  ||  /  ;           ;  \  \| \  /
 ||   ||  | ;             ; |  ||  ||
 %%   %%  | ;             ; |  %%  %%
 %%   %%  \  ;           ;  /  %%  %%
 %%   %%   \  '.       .'  /   %%  %%
 %%   %%    '.  `-.,.-'  .'    %%  %%
 %%   %%      '-.,___,.-'      %%  %%
"#;