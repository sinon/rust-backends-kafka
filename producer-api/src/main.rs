use axum::{
    body::Bytes,
    extract::State,
    http::{header, HeaderValue, StatusCode},
    routing::post,
    Json, Router,
};
use cloudevents::{
    binding::rdkafka::{FutureRecordExt, MessageRecord},
    Event,
};
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::{config::ClientConfig, util::Timeout};
use serde::Serialize;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::{
    timeout::TimeoutLayer,
    trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer},
    LatencyUnit, ServiceBuilderExt,
};
use tracing_subscriber::prelude::__tracing_subscriber_SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

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
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", "localhost:9092")
        .set("message.timeout.ms", "5000")
        .create()
        .expect("Producer creation error");

    let app = Router::new()
        .route("/event", post(create_event))
        .with_state(producer)
        .layer(middleware);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::warn!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct EventResponse {
    success: bool,
    partition: i32,
    offset: i64,
}

#[axum::debug_handler]
async fn create_event(
    State(producer): State<FutureProducer>,
    Json(event): Json<Event>,
) -> (StatusCode, Json<EventResponse>) {
    let message_record =
        MessageRecord::from_event(event).expect("error while serializing the event");
    let (i, j) = producer
        .send(
            FutureRecord::to("user-behaviour.events")
                .key("some_event")
                .message_record(&message_record),
            Timeout::After(Duration::from_secs(1)),
        )
        .await
        .unwrap();
    tracing::info!(
        "message published to user-bheaviour.events topic, {:?} {:?}",
        i,
        j
    );
    (
        StatusCode::OK,
        Json(EventResponse {
            success: true,
            partition: i,
            offset: j,
        }),
    )
}
