
pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let response = format!(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <style>
                body {{ 
                    font-family: 'Courier New', monospace; 
                    background-color: #1a1a1a; 
                    color: #e0e0e0; 
                    padding: 2rem;
                }}
                .container {{ 
                    max-width: 800px; 
                    margin: 0 auto;
                }}
                pre {{ 
                    color: #7f7f7f; 
                    margin-bottom: 2rem;
                }}
                .title {{ 
                    color: #00ff00; 
                    font-size: 2rem; 
                    margin-bottom: 1.5rem;
                }}
                .endpoints {{ 
                    margin-left: 2rem; 
                    line-height: 1.6;
                }}
                .method {{
                    color: #4CAF50;
                    font-weight: bold;
                }}
                .method-post {{ color: #FF9800; }}
                .method-put {{ color: #2196F3; }}
                .method-delete {{ color: #F44336; }}
                .param {{ 
                    color: #9C27B0; 
                    font-style: italic;
                }}
                .warning {{ 
                    color: #FF5722; 
                    font-weight: bold;
                    margin-top: 2rem;
                }}
                .example {{ 
                    color: #607D8B; 
                    margin-left: 1rem;
                }}
            </style>
        </head>
        <body>
            <div class="container">
                <pre>{ASCII}</pre>
                
                <div class="title">Restaurant API</div>
                
                <div class="endpoints">
                    <strong>Endpoints:</strong><br><br>
                    
                    <span class="method">GET</span>  <a href="/restaurants">/restaurants</a><br>
                    <span class="method method-post">POST</span> /restaurants<br>
                    
                    <span class="method">GET</span>  /restaurants/<span class="param">id</span><br>
                    <span class="method method-put">PUT</span>    /restaurants/<span class="param">id</span><br>
                    <span class="method method-delete">DELETE</span> /restaurants/<span class="param">id</span><br>
                    
                    <span class="method">GET</span>  /restaurants/<span class="param">id</span>/comments<br>
                    <span class="method method-post">POST</span> /restaurants/<span class="param">id</span>/comments<br>
                    <span class="example">example: {{ "name": "ali", "text": "awesome!" }}</span><br>
                    
                    <span class="method method-put">PUT</span>    /restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/likes/add<br>
                    <span class="method method-put">PUT</span>    /restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/dislikes/add<br>
                    <span class="method method-put">PUT</span>    /restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/likes/remove<br>
                    <span class="method method-put">PUT</span>    /restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/dislikes/remove<br>
                    
                    <span class="method">GET</span>  /restaurants/city/<span class="param">tag</span><br>
                    <span class="method">GET</span>  /restaurants/tag/<span class="param">city</span><br>
                    
                    <div class="warning">UNDER DEVELOPMENT!</div>
                </div>
            </div>
        </body>
        </html>
    "#,
        ASCII = ASCII
    );

    Ok(warp::reply::with_status(
        warp::reply::html(response),
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
