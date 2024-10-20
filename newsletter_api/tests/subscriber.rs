use std::collections::HashMap;

use hyper::StatusCode;
use sqlx::PgPool;

mod common;

#[sqlx::test]
async fn create_subscriber_works(db: PgPool) -> sqlx::Result<()> {
    let address = common::spawn_app().await;

    let mut map = HashMap::new();
    map.insert("email", "test@example.com");
    map.insert("name", "Joe B");

    let client = reqwest::Client::new();
    let response = client
        .post(&format!("{}/api/subscriber", &address))
        .json(&map)
        .send()
        .await
        .expect("Failed to execute request.");
    assert_eq!(response.status(), StatusCode::CREATED);

    let resp_json: serde_json::Value = serde_json::from_slice(&response.bytes().await.unwrap())
        .expect("failed to read response body as json");

    assert_eq!(resp_json["name"], "Joe B");
    assert_eq!(resp_json["email"], "test@example.com");

    let _subcriber_id = common::expect_uuid(&resp_json["id"]);
    // todo!("Figure out setup/teardown of DB between runs");

    Ok(())
}

#[sqlx::test]
async fn create_subsciber_fails(db: PgPool) -> sqlx::Result<()> {
    // todo!("Add test that tries to create a duplicate subscriber");
    Ok(())
}
