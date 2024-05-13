// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
mod common;

#[tokio::test]
async fn healthz_works() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/healthz", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
