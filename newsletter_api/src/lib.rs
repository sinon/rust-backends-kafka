use std::net::SocketAddr;

use axum::{
    http::StatusCode,
    routing::{get, IntoMakeService},
    Error, Router,
};
use hyper::server::conn::AddrIncoming;

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn generate_routes() -> Router {
    Router::new().route("/healthcheck", get(health_check))
}

pub fn run(port: u16) -> Result<axum::Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server: hyper::Server<hyper::server::conn::AddrIncoming, IntoMakeService<Router>> =
        axum::Server::bind(&addr).serve(generate_routes().into_make_service());
    Ok(server)
}
