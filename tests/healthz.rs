// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-

#[tokio::test]
async fn healthz_works() {
    spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/healthz")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let server = zero2prod::run().expect("Failed to bind address.");

    let _ = tokio::spawn(server);
}
