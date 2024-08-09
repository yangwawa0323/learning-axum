use axum::{routing::get, Router};

pub fn create_route() -> Router {
    let user_app = Router::new()
        .route("/hi", get(say_hi))
        .route("/welcome", get(welcome).post(welcome));
    user_app
}

async fn welcome() -> String {
    "Welcome!!!".to_string()
}

async fn say_hi() -> String {
    "Hi!".to_string()
}
