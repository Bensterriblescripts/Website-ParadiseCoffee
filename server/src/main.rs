use axum::{
    response::Html,
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(home))
        .route("/api", get(api)
    );

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
// HTML
async fn home() -> Html<String> {
    Html(format!("
        <html>
            <head>
                <style src='/htmx-1.9.5.min.js'></style>
                <style>
                    .fade-me-out.htmx-swapping {{
                        opacity: 0;
                        transition: opacity 1s ease-out;
                    }}
               </style> 
            </head>
            <h3>Homepage</h3>
            <button class='fade-me-out' hx-swap='outerHTML swap:1s'>Please Work</button>

        </html>
    "))
}


// API
async fn api() -> Html<String> {
    Html(format!("
        <html>
            <head>
                <style src='/htmx-1.9.5.min.js'></style>
            </head>
            <div><u>API</u></div>
        </html>
    "))
}