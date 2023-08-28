use std::net::SocketAddr;

use axum::response::Html;
use axum::Router;

#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/", get(hello)
    );

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("LISTENING on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes_hello.into_make_service())
}

async fn hello () {
    Html("Hello")
}