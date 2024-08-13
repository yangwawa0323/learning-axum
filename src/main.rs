use learning_axum::users::middleware::MyLayer;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut app = axum::Router::new();

    let user_routes = learning_axum::users::interactive::create_route();

    let json_map_routes = learning_axum::users::map_json::create_routes();

    let path_demo_routes = learning_axum::users::path_extractor::create_routes();

    let shared_data_routes = learning_axum::users::extension::create_routes();

    app = app
        .merge(user_routes)
        .merge(shared_data_routes)
        .merge(json_map_routes)
        .merge(path_demo_routes)
        .layer(MyLayer);

    // listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
