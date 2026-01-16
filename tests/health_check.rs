use std::net::TcpListener;
use reqwest::StatusCode;
use zero2prod::run;

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind to test port");
    let port = listener.local_addr().expect("Unable to get address of server").port();
    let server = run(listener).expect("Failed to start server");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}

#[tokio::test]
async fn health_check_test() {
    // Arrange
    let addr = spawn_app();
    let client = reqwest::Client::new();

    // Act
    let response = client
        .get(format!("{}/health_check", addr))
        .send()
        .await
        .expect("Failed to send request");

    // Assert
    assert_eq!(StatusCode::OK, response.status());
    assert_eq!(Some(0), response.content_length());
}
