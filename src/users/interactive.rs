use axum::{
    body::Body,
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use http_body_util::BodyExt;

pub fn create_route() -> Router {
    let user_app = Router::new()
        .route("/uppercase", post(echo_uppercase))
        .route("/hi_resp_obj", get(say_hi_vec_response_obj))
        .route("/hi_resp", get(say_hi_vec_response))
        .route("/hi3", get(say_hi_vec))
        .route("/hi2", get(say_hi2))
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

async fn say_hi2() -> &'static [u8] {
    "Hello".as_bytes()
}

async fn say_hi_vec() -> Vec<u8> {
    let resp = "Hello 5117xue.com".as_bytes().to_vec();
    resp
}

async fn say_hi_vec_response() -> impl IntoResponse {
    let resp = "Hello 51cloudclass.com".as_bytes().to_vec();
    resp
}

async fn say_hi_vec_response_obj() -> Response {
    let resp = "Hello 51cloudclass.com 5117xue.com".as_bytes().to_vec();
    resp.into_response()
}

async fn echo_uppercase(req: Body) -> impl IntoResponse {
    let frame = req.map_frame(|frame| {
        frame.map_data(|data| {
            data.iter()
                .map(|byte| byte.to_ascii_uppercase())
                .collect::<axum::body::Bytes>()
        })
    });
    let resp = Response::new(frame);
    resp
}
