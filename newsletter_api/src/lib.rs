use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::{get, post, IntoMakeService},
    Error, Json, Router,
};
use hyper::server::conn::AddrIncoming;
use serde::{Deserialize, Serialize};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn create_subscriber(
    Json(payload): Json<CreateSubscriber>,
) -> (StatusCode, Json<Subscriber>) {
    let subscriber = Subscriber {
        id: 123,
        name: payload.name,
        email: payload.email,
    };
    (StatusCode::OK, Json(subscriber))
}

#[derive(Deserialize)]
struct CreateSubscriber {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct Subscriber {
    id: u64,
    name: String,
    email: String,
}

pub fn generate_routes() -> Router {
    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/api/subscriber", post(create_subscriber))
}

pub fn run(port: u16) -> Result<axum::Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server: hyper::Server<hyper::server::conn::AddrIncoming, IntoMakeService<Router>> =
        axum::Server::bind(&addr).serve(generate_routes().into_make_service());
    Ok(server)
}
