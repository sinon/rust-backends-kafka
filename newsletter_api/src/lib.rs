use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post, IntoMakeService},
    Error, Json, Router,
};
use hyper::server::conn::AddrIncoming;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, types::Uuid, FromRow, PgPool};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

async fn create_subscriber(
    State(_state): State<AppState>,
    Json(payload): Json<CreateSubscriber>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    match sqlx::query_as!(
        Subscriber,
        "INSERT INTO subscriber (name, email) VALUES ($1, $2) RETURNING id, name, email",
        &payload.name,
        &payload.email,
    )
    .fetch_one(&_state.pool)
    .await
    {
        Ok(subscriber) => Ok((StatusCode::CREATED, Json(subscriber))),
        Err(e) => Err((StatusCode::BAD_REQUEST, e.to_string())),
    }
}

#[derive(Deserialize)]
struct CreateSubscriber {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, FromRow)]
struct Subscriber {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

pub fn generate_routes(pool: PgPool) -> Router {
    let state = AppState { pool };
    Router::new()
        .route("/healthcheck", get(health_check))
        .route("/api/subscriber", post(create_subscriber))
        .with_state(state)
}

pub async fn run(port: u16) -> Result<axum::Server<AddrIncoming, IntoMakeService<Router>>, Error> {
    let db_connection_str = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:password@localhost".to_string());

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .unwrap();
    sqlx::migrate!().run(&pool).await.unwrap();
    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    let server: hyper::Server<hyper::server::conn::AddrIncoming, IntoMakeService<Router>> =
        axum::Server::bind(&addr).serve(generate_routes(pool).into_make_service());
    Ok(server)
}
