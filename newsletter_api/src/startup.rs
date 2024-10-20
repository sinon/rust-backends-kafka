// use crate::routes::{health_check, subscribe};
// use actix_web::dev::Server;
// use actix_web::web::Data;
// use actix_web::{web, App, HttpServer};
// use sqlx::PgPool;
// use std::net::TcpListener;

// pub fn run(listener: TcpListener, db_pool: PgPool) -> Result<Server, std::io::Error> {
//     let db_pool = Data::new(db_pool);
//     let server = HttpServer::new(move || {
//         App::new()
//             .route("/health_check", web::get().to(health_check))
//             .route("/subscriptions", web::post().to(subscribe))
//             .app_data(db_pool.clone())
//     })
//     .listen(listener)?
//     .run();
//     Ok(server)
// }

use std::net::SocketAddr;

use axum::{
    routing::{get, IntoMakeService},
    Error, Router,
};
use hyper::server::conn::AddrIncoming;
use sqlx::PgPool;

use crate::routes::{create_subscriber, health_check};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

pub fn generate_routes(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/api/subscriber", axum::routing::post(create_subscriber))
        .with_state(state)
}

pub async fn run(
    port: u16,
    db_pool: PgPool,
) -> Result<axum::Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    // let db_connection_str = std::env::var("DATABASE_URL")
    //     .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    // let pool = PgPoolOptions::new()
    //     .max_connections(5)
    //     .acquire_timeout(Duration::from_secs(3))
    //     .connect(&db_connection_str)
    //     .await
    //     .unwrap();
    // sqlx::migrate!().run(&pool).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server: hyper::Server<hyper::server::conn::AddrIncoming, IntoMakeService<Router>> =
        axum::Server::bind(&addr).serve(generate_routes(db_pool).into_make_service());
    Ok(server)
}
