use std::sync::Arc;

use axum::{
    extract::Request,
    http::StatusCode,
    middleware::{from_fn, Next},
    response::{IntoResponse, Response},
    routing::get,
    Extension, Router,
};

pub struct AppState {
    pub db: String,
}

impl AppState {
    fn new() -> Self {
        Self {
            db: "This is a share database connection".to_string(),
        }
    }

    fn get_conn(&self) -> String {
        self.db.clone()
    }
}

pub fn create_routes() -> Router {
    let db_data: Arc<AppState> = Arc::new(AppState::new());

    Router::new()
        .route("/shared-data-1", get(shared_data_1))
        .route("/shared-data-2", get(shared_data_2))
        .layer(Extension(db_data))
        .route_layer(from_fn(custom_middleware))
}

async fn shared_data_1(Extension(db): Extension<Arc<AppState>>) -> String {
    format!("get shared data from shared_data_1() : {}", db.get_conn())
}

async fn shared_data_2(Extension(db): Extension<Arc<AppState>>) -> String {
    format!("shared_data_2() get the shared data : {}", db.get_conn())
}

async fn custom_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    if req.headers().get("x-custom-header").is_none() {
        return Err(StatusCode::BAD_REQUEST);
    }
    Ok(next.run(req).await)
}
