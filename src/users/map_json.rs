use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};

pub fn create_routes() -> Router {
    Router::new().route("/json_demo", post(map_json_struct))
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    title: String,
    detail: Option<String>,
    is_completed: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoResponse {
    title: String,
    message: String,
}

// json request mapping to struct
async fn map_json_struct(Json(req): Json<Todo>) -> Json<TodoResponse> {
    let resp = TodoResponse {
        title: req.title,
        message: "Post data successfully".to_string(),
    };
    Json(resp)
}
