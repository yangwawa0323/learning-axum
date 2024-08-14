use axum::{
    async_trait,
    extract::{rejection::JsonRejection, FromRequest, Request},
    http::StatusCode,
    routing::post,
    Json, RequestExt, Router,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use validator::Validate;

pub fn create_routes() -> Router {
    Router::new().route("/validate-json", post(custom_json_extractor))
}

async fn custom_json_extractor(js_data: RequestUser) -> Json<RequestUser> {
    tracing::debug!("{js_data:#?}");
    Json(js_data)
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RequestUser {
    #[validate(email(message = "please input user email"))]
    pub name: String,
    #[validate(length(min = 8, message = "password at least 8 characters"))]
    pub password: String,
    #[validate(range(min = 18, max = 60, message = "age is out of range"))]
    pub age: u32,
}

#[async_trait]
impl<S> FromRequest<S> for RequestUser
where
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    // required method
    async fn from_request(req: Request, _state: &S) -> Result<Self, Self::Rejection> {
        let Json(user): Json<RequestUser> = req.extract().await.map_err(|err: JsonRejection| {
            (
                StatusCode::BAD_REQUEST,
                json!({
                    "message": format!("{}", err)
                })
                .to_string(),
            )
        })?;

        if let Err(err) = user.validate() {
            return Err((
                StatusCode::BAD_REQUEST,
                json!({
                    "message": format!("{}", err)
                })
                .to_string(),
            ));
        }
        Ok(user)
    }
}
