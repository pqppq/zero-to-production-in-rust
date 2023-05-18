use reqwest::header::HeaderValue;
use std::collections::HashSet;

use crate::helpers::assert_is_redirect_to;
use crate::helpers::spawn_app;

#[tokio::test]
async fn an_error_flash_message_is_set_on_failure() {
    // Arrange
    let app = spawn_app().await;

    // Act - Try to login
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

    // Act2 - Follow the redirect
    let html_page = app.get_login_html().await;

    // Assert2
    assert!(html_page.contains(r#"<p><i>Authentication failed</i></p>"#));

    // Act3 - Reload the login page
    let html_page = app.get_login_html().await;

    // Assert3
    assert!(!html_page.contains(r#"Authentication failed"#));
}

#[tokio::test]
async fn redirect_to_admin_dashboard_after_login_success() {
    // Arrange
    let app = spawn_app().await;

    // Act - Login
    let login_body = serde_json::json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password,
    });
    let response = app.post_login(&login_body).await;

    // Assert
    assert_is_redirect_to(&response, "/admin/dashboard");

    // Act - Follow the redirect
    let html_page = app.get_admin_dashboard().await;

    // Assert2
    assert!(html_page.contains(&format!("Welcome {}!", &app.test_user.username)));
}
