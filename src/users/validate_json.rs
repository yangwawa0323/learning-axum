use axum::{
    async_trait,
    extract::{FromRequest, Request},
    http::StatusCode,
    routing::post,
    Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

pub fn create_routes() -> Router {
    Router::new().route("/validate-json", post(custom_json_extractor))
}

async fn custom_json_extractor(js_data: RequestUser) {
    tracing::debug!("{js_data:#?}");
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RequestUser {
    pub name: String,
    pub password: String,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    // required method
    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        if rand::random::<bool>() {
            Ok(RequestUser {
                name: "abc".to_string(),
                password: String::from("secret"),
            })
        } else {
            Err((
                StatusCode::BAD_REQUEST,
                json!({
                    "message" : "request is not correct!"
                })
                .to_string(),
            ))
        }
    }
}
