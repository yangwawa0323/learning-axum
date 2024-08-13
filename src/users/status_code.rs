use axum::{http::StatusCode, routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/status-code", get(status_code))
        .route("/status-code-with-content", get(status_code_with_content))
        .route("/status-code-with-result", get(status_code_with_result))
}

async fn status_code() -> StatusCode {
    StatusCode::NO_CONTENT
}

async fn status_code_with_content() -> (StatusCode, String) {
    // searching in database, but found the record.
    (StatusCode::BAD_REQUEST, "Illegal parameters".to_string())
}

async fn status_code_with_result() -> Result<String, StatusCode> {
    // Ok("It is a demo page".to_string())
    Err(StatusCode::INTERNAL_SERVER_ERROR)
}
