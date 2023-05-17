use reqwest::header::HeaderValue;
use std::collections::HashSet;

use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act
    let login_body = serde_json::json!({
        "username": "username",
        "password": "password",
    });
    let response = app.post_login(&login_body).await;
    let cookies: HashSet<_> = response
        .headers()
        .get_all("Set-Cookie")
        .into_iter()
        .collect();

    // Assert
    // status 303: See Other
    assert_is_redirect_to(&response, "/login");
    assert!(cookies.contains(&HeaderValue::from_str("_flash=Authentication failed").unwrap()));

    // Act2
    let html_page = app.get_login_html().await;

    // Assert2
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // Act3
    let html_page = app.get_login_html().await;

    // Assert3
    assert!(!html_page.contains(r#"Authentication failed"#));
}
