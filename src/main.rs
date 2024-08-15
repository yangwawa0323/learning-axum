use std::net::SocketAddr;

use learning_axum::users::middleware::MyLayer;
use time::macros::format_description;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::fmt::time::LocalTime;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .pretty()
        .compact()
        .with_target(false)
        .with_timer(LocalTime::new(format_description!(
            "[year]-[month]-[day] [hour]:[minute]:[second]"
        )))
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let mut app = axum::Router::new().fallback_service(
        ServeDir::new("static_folder").not_found_service(ServeFile::new("static/404.html")),
    );

    let user_routes = learning_axum::users::interactive::create_route();

    let json_map_routes = learning_axum::users::map_json::create_routes();

    let path_demo_routes = learning_axum::users::path_extractor::create_routes();

    let shared_data_routes = learning_axum::users::extension::create_routes();

    let status_code_routes = learning_axum::users::status_code::create_routes();

    let validate_json_routes = learning_axum::users::validate_json::create_routes();

    app = app
        .merge(user_routes)
        .merge(shared_data_routes)
        .merge(json_map_routes)
        .merge(validate_json_routes)
        .merge(status_code_routes)
        .merge(path_demo_routes)
        .layer(MyLayer);

    // listener
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
