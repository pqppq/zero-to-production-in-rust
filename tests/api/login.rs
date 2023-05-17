use crate::helpers::spawn_app;
use crate::helpers::assert_is_redirect_to;

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

    // Assert
    // status 303: See Other
    assert_is_redirect_to(&response, "/login")
}
