use std::net::SocketAddr;
use serde::Deserialize;

use axum::Json;
use axum::response::IntoResponse;
use axum::response::Html;
use axum::routing::get;
use axum::Router;

#[tokio::main]
async fn main() {

    // Endpoint
    let routes = Router::new().route("/search", get(search));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("--> Listening on {addr}\n");
    axum::Server::bind(&addr)
        .serve(routes.into_make_service())
        .await
        .unwrap();
}

// API
#[derive(Debug, Deserialize)]
struct SearchParams {
    dn: Option<String>,
    filter: Option<String>,
    attributes: Option<Vec<String>>,
}
async fn search (Json(params): Json<SearchParams>) -> impl IntoResponse {

    println!("->> {:<12} - Search - {params:?}", "Handler");

    let ldap_dn = params.dn.as_deref().unwrap_or("cn=admin,dc=paradisecoffee,dc=cafe");
    let ldap_filter = params.filter.as_deref().unwrap_or("sn=*");
    let ldap_attr: Vec<String> = params.attributes.unwrap_or(Vec::new());

    Html(format!("DN: {ldap_dn}\n\nFilter: {ldap_filter}\n\nAttributes: {ldap_attr:?}"))
}