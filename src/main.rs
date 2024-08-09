use axum::routing::get;

#[tokio::main]
async fn main() {
    let app = axum::Router::new()
        .route("/", get(|| async { "Hello 5117xue.com" }))
        .route("/hi", get(say_hi));
    // ;

    // listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn say_hi() -> String {
    "Hi!!!!!!!!!".to_string()
}
