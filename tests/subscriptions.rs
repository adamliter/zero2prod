// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
mod common;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();

    let body = "name=Adam%20Liter&email=io%40adamliter.org";
    let response = client
        .post(&format!("{}/subscriptions", &address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let address = common::spawn_app();
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Adam%20Liter", "missing the email"),
        ("email=io%40adamliter.org", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &address))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            400,
            response.status().as_u16(),
            "The API did not fail with 400 Bad Request when the payload was {}.",
            error_message
        )
    }
}
