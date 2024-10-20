use sqlx::types::Uuid;
use tokio::net::TcpListener;

#[cfg(test)]
pub async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    // We retrieve the port assigned to us by the OS
    let port = listener.local_addr().unwrap().port();
    // Drop listener to free-up the selected port
    drop(listener);
    let server = newsletter_api::run(port).await.unwrap();
    let _ = tokio::spawn(server);
    format!("http://127.0.0.1:{}", port)
}

#[track_caller]
#[cfg(test)]
pub fn expect_string(value: &serde_json::Value) -> &str {
    value
        .as_str()
        .unwrap_or_else(|| panic!("expected string, got {value:?}"))
}

#[track_caller]
#[cfg(test)]
pub fn expect_uuid(value: &serde_json::Value) -> Uuid {
    expect_string(value)
        .parse::<Uuid>()
        .unwrap_or_else(|e| panic!("failed to parse UUID from {value:?}: {e}"))
}
