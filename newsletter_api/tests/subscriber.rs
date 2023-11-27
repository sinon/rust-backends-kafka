use std::collections::HashMap;

mod common;

#[tokio::test]
async fn create_subscriber_works() {
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
    assert!(response.status().is_success());
    assert_eq!(
        response.text().await.unwrap(),
        "{\"id\":123,\"name\":\"Joe B\",\"email\":\"test@example.com\"}"
    )
}
