use axum::{extract::Path, response::IntoResponse, routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/users/:user_id", get(get_user_from_id))
        .route(
            "/category/:cat_id/product/:pro_id",
            get(get_product_from_category),
        )
}

async fn get_user_from_id(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("get user id from path :  {user_id:}")
}

async fn get_product_from_category(Path((cat_id, pro_id)): Path<(i32, i32)>) -> String {
    format!("searching category id : {cat_id}, product id: {pro_id}")
}
