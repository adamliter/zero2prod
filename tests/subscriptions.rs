// -*- mode: rustic; coding: utf-8; fill-column: 88; -*-
mod common;

#[tokio::test]
async fn subscribe_returns_a_200_for_valid_form_data() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=Adam%20Liter&email=io%40adamliter.org";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions;")
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "io@adamliter.org");
    assert_eq!(saved.name, "Adam Liter");
}

#[tokio::test]
async fn subscribe_returns_a_400_when_data_is_missing() {
    let app = common::spawn_app().await;
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=Adam%20Liter", "missing the email"),
        ("email=io%40adamliter.org", "missing the name"),
        ("", "missing both"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &app.address))
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
