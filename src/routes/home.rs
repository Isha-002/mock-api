pub async fn home() -> Result<impl warp::Reply, warp::Rejection> {
    let response = r#"
    <!DOCTYPE html>
    <html>
    <head>
        <style>
            :root {
                --primary: #00ff9d;
                --background: #0d1117;
                --surface: #161b22;
                --border: #30363d;
            }

            body { 
                font-family: 'SF Mono', 'Courier New', monospace; 
                background-color: var(--background); 
                color: #c9d1d9; 
                margin: 0;
                line-height: 1.6;
            }

            .container { 
                max-width: 800px; 
                margin: 0 auto;
                padding: 2rem 1.5rem;
            }

            .header {
                border-bottom: 1px solid var(--border);
                padding-bottom: 1.5rem;
                margin-bottom: 2rem;
            }

            .title { 
                color: var(--primary); 
                font-size: 2.5rem; 
                margin: 0;
                letter-spacing: -0.05em;
            }

            .endpoints {
                background: var(--surface);
                border: 1px solid var(--border);
                border-radius: 8px;
                padding: 1.5rem;
            }

            .endpoint {
                display: grid;
                grid-template-columns: 80px 1fr;
                gap: 1rem;
                padding: 0.75rem 1rem;
                border-bottom: 1px solid var(--border);
            }

            .endpoint:last-child {
                border-bottom: none;
            }

            .method {
                font-weight: 600;
                text-transform: uppercase;
                font-size: 0.9em;
                padding: 2px 8px;
                border-radius: 4px;
                width: fit-content;
                height: fit-content;
            }

            .method-get { color: #58a6ff; border: 1px solid #58a6ff; }
            .method-post { color: #ffa657; border: 1px solid #ffa657; }
            .method-put { color: #bd8cff; border: 1px solid #bd8cff; }
            .method-delete { color: #f85149; border: 1px solid #f85149; }

            a {
                color: var(--primary);
                text-decoration: none;
            }

            a:hover {
                text-decoration: underline;
            }

            .param {
                color: #79c0ff;
                font-style: italic;
            }

            .example {
                background: var(--background);
                border: 1px solid var(--border);
                border-radius: 6px;
                padding: 1rem;
                margin: 1rem 0;
                font-size: 0.9em;
                overflow-x: auto;
            }

            .warning {
                background: #ffa65722;
                border: 1px solid #ffa657;
                color: #ffa657;
                padding: 1rem;
                border-radius: 6px;
                margin-top: 2rem;
                display: flex;
                align-items: center;
                gap: 0.75rem;
            }

            .warning::before {
                content: "⚠️";
            }
        </style>
    </head>
    <body>
        <div class="container">
            <div class="header">
                <div class="title">RESTaurant API</div>
            </div>

            <div class="endpoints">
                <div class="endpoint"><span class="method method-get">GET</span><div><a href="/restaurants">/restaurants</a></div></div>
                <div class="endpoint"><span class="method method-post">POST</span><div>/restaurants</div></div>

                <div class="endpoint"><span class="method method-get">GET</span><div>/restaurants/<span class="param">id</span></div></div>
                <div class="endpoint"><span class="method method-put">PUT</span><div>/restaurants/<span class="param">id</span></div></div>
                <div class="endpoint"><span class="method method-delete">DELETE</span><div>/restaurants/<span class="param">id</span></div></div>

                <div class="endpoint"><span class="method method-get">GET</span><div>/restaurants/<span class="param">id</span>/comments</div></div>
                <div class="endpoint"><span class="method method-post">POST</span><div>/restaurants/<span class="param">id</span>/comments</div></div>
                <div class="example">
                    curl -X POST http://localhost:4444/restaurants/1/comments \<br>
                    -H "Content-Type: application/json" \<br>
                    -d '{"name": "ali", "text": "awesome!"}'
                </div>

                <div class="endpoint"><span class="method method-put">PUT</span><div>/restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/likes/add</div></div>
                <div class="endpoint"><span class="method method-put">PUT</span><div>/restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/dislikes/add</div></div>
                <div class="endpoint"><span class="method method-put">PUT</span><div>/restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/likes/remove</div></div>
                <div class="endpoint"><span class="method method-put">PUT</span><div>/restaurants/<span class="param">id</span>/comments/<span class="param">comment_id</span>/dislikes/remove</div></div>

                <div class="endpoint"><span class="method method-get">GET</span><div>/restaurants/city/<span class="param">city</span></div></div>
                <div class="endpoint"><span class="method method-get">GET</span><div>/restaurants/tag/<span class="param">tag</span></div></div>

                <div class="endpoint"><span class="method method-post">POST</span><div>/restaurants/<span class="param">id</span>/upload</div></div>
                <div class="example">
                    curl -X POST http://localhost:4444/restaurants/1/upload \<br>
                    -F "file=@/file/path"
                </div>

                <div class="endpoint"><span class="method method-post">POST</span><div>/registration</div></div>
                <div class="example">
                    curl -X POST http://localhost:4444/registration \<br>
                    -H "Content-Type: application/json" \<br>
                    -d '{"email": "test@example.com", "password": "secret123", "phone_number": "1234567890", "role": "customer"}'
                </div>

                <div class="endpoint"><span class="method method-post">POST</span><div>/login</div></div>
                <div class="example">
                    curl -X POST http://localhost:4444/login \<br>
                    -H "Content-Type: application/json" \<br>
                    -d '{"email": "test@example.com", "password": "secret123"}'
                </div>

                <div class="warning">
                    This API is currently under active development. Endpoints may change without notice.
                </div>
            </div>
        </div>
    </body>
    </html>
"#;


    Ok(warp::reply::with_status(
        warp::reply::html(response),
        warp::http::StatusCode::OK,
    ))
}