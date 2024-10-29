use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};

use crate::startup::AppState;

#[derive(Deserialize)]
pub struct CreateSubscriber {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, FromRow)]
struct Subscriber {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

pub async fn create_subscriber(
    State(_state): State<AppState>,
    Json(payload): Json<CreateSubscriber>,
) -> Result<impl IntoResponse, impl IntoResponse> {
    tracing::info!("Creating subscriber.");
    match sqlx::query_as!(
        Subscriber,
        "INSERT INTO subscriber (name, email) VALUES ($1, $2) RETURNING id, name, email",
        &payload.name,
        &payload.email,
    )
    .fetch_one(&_state.pool)
    .await
    {
        Ok(subscriber) => {
            tracing::info!("New Subscriber sucessfully created");
            Ok((StatusCode::CREATED, Json(subscriber)))
        }
        Err(e) => {
            tracing::error!("Failed to execute query: {:?}", e);
            Err((StatusCode::BAD_REQUEST, e.to_string()))
        }
    }
}
