use sqlx::PgPool;

mod common;

#[sqlx::test]
async fn health_check_works(db: PgPool) {
    let address = common::spawn_app().await;

    let client = reqwest::Client::new();
    let response = client
        .get(&format!("{}/healthcheck", &address))
        .send()
        .await
        .expect("Failed to execute request.");
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
