#![allow(unused)]
use std::net::SocketAddr;

use axum::Router;
use axum::routing::get;
use axum::response::Html;
#[tokio::main]
async fn main() {
    let routes_hello = Router::new().route(
        "/hello",
        get(|| async{ Html("HelloWorld") }),
    );
    let addr = SocketAddr::from(([127,0,0,1], 8080));
    axum::Server::bind(&addr)
    .serve(routes_hello.into_make_service())
    .await
    .unwrap();
}
