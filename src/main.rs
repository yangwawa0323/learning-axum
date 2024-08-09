#[tokio::main]
async fn main() {
    let mut app = axum::Router::new();

    let user_routes = learning_axum::users::interactive::create_route();

    app = app.merge(user_routes);

    // listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
