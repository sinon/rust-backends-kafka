use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderValue, StatusCode},
    routing::{post},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, sync::Arc, time::Duration};
use std::sync::Mutex;
use tower::ServiceBuilder;
use tower_http::{
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

use kafka::producer::{Producer, Record, RequiredAcks};

#[derive(Clone)]
struct AppState {
    kafka_producer: Arc<Mutex<Producer>>,
    // kafka_producer: String,
}

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                // axum logs rejections from built-in extractors with the `axum::rejection`
                // target, at `TRACE` level. `axum::rejection=trace` enables showing those events
                "api=debug,tower_http=debug,axum::rejection=trace".into()
            }),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();
    let sensitive_headers: Arc<[_]> = vec![header::AUTHORIZATION, header::COOKIE].into();

    // Build our middleware stack
    let middleware = ServiceBuilder::new()
        // Mark the `Authorization` and `Cookie` headers as sensitive so it doesn't show in logs
        .sensitive_request_headers(sensitive_headers.clone())
        // Add high level tracing/logging to all requests
        .layer(
            TraceLayer::new_for_http()
                .on_body_chunk(|chunk: &Bytes, latency: Duration, _: &tracing::Span| {
                    tracing::trace!(size_bytes = chunk.len(), latency = ?latency, "sending body chunk")
                })
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_response(DefaultOnResponse::new().include_headers(true).latency_unit(LatencyUnit::Micros)),
        )
        .sensitive_response_headers(sensitive_headers)
        // Set a timeout
        .layer(TimeoutLayer::new(Duration::from_secs(10)))
        // Box the response body so it implements `Default` which is required by axum
        .map_response_body(axum::body::boxed)
        // Compress responses
        .compression()
        // Set a `Content-Type` if there isn't one already.
        .insert_response_header_if_not_present(
            header::CONTENT_TYPE,
            HeaderValue::from_static("application/octet-stream"),
        );
    // TODO: Replace with actual producer
    let producer = Producer::from_hosts(vec!("localhost:9092".to_owned()))
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One).create().unwrap();
    let shared_state = AppState {
        // kafka_producer: "SET".to_string()
        kafka_producer: Arc::new(Mutex::new(producer)),
    };

    let app = Router::new()
        .route("/event", post(create_event))
        .with_state(shared_state)
        .layer(middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::warn!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Deserialize, Serialize)]
struct CreateEvent {
    event_type: String,
}

#[derive(Serialize)]
struct Event {
    event_type: String,
}

async fn create_event(
    State(_state): State<AppState>,
    Json(payload): Json<CreateEvent>,
) -> (StatusCode, Json<Event>) {
    let j = serde_json::to_string(&payload);
    match j {
        Ok(j) => {
            let mut kp = _state.kafka_producer.lock().expect("mutex was poisoned");
            kp.send(&Record::from_value("user-behaviour.events", j)).unwrap()
        },
        Err(_j) => {}
    }


    let event = Event {
        event_type: payload.event_type,
    };
    (StatusCode::CREATED, Json(event))
}
