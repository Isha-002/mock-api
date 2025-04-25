pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let response = format!("{ASCII}
    \n\nRestaurant Api 
    \n\nEndpoints: 
    \n\n/restaurants (get)
    \n/restaurants (post)
    \n/restaurants/id (get)
    \n/restaurants/id (put)
    \n/restaurants/id (delete)
    \n/restaurants/id/comments (get)
    \n/restaurants/id/comments (post) example: {{ name: ali, text: awesome! }}
    \n/restaurants/id/comments/id/likes/add   (put) -> add a comment like
    \n/restaurants/id/comments/id/dislikes/add   (put) -> remove a comment like
    \n/restaurants/id/comments/id/likes/remove   (put) -> add a comment dislike
    \n/restaurants/id/comments/id/dislikes/remove   (put) -> remove a comment dislike
    \n\nUNDER DEVELOPMENT!");
    Ok(warp::reply::with_status(
        response,
        warp::http::StatusCode::OK,
    ))
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
