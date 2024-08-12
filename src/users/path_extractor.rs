use axum::{
    extract::{Path, Query},
    http::HeaderMap,
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};

pub fn create_routes() -> Router {
    Router::new()
        .route("/list-header", get(list_header))
        .route("/users/:user_id", get(get_user_from_id))
        .route(
            "/category/:cat_id/product/:pro_id",
            get(get_product_from_category),
        )
        .route("/list-products", get(list_products))
}

async fn get_user_from_id(Path(user_id): Path<i32>) -> impl IntoResponse {
    format!("get user id from path :  {user_id:}")
}

async fn get_product_from_category(Path((cat_id, pro_id)): Path<(i32, i32)>) -> String {
    format!("searching category id : {cat_id}, product id: {pro_id}")
}

#[derive(Debug, Serialize, Deserialize)]
struct Pagination {
    //    access `/list-products?amount-per-page=25&page=17`
    #[serde(rename = "amount-per-page")]
    amount_per_page: Option<u32>, // by default : 10
    page: Option<u32>, // by default : 1
}

async fn list_products(Query(pagination): Query<Pagination>) -> String {
    let page = pagination.page.unwrap_or(1);
    let amount = pagination.amount_per_page.unwrap_or(10);
    format!("You are list {amount} products at page: {page}")
}

async fn list_header(hm: HeaderMap) -> String {
    if let Some(value) = hm.get("x-custom-header") {
        println!("Custom header: {}", value.to_str().unwrap());
    }

    let mut response = String::from(""); // initial empty `String`

    hm.iter()
        .map(|(name, value)| {
            response.push_str(&format!("Name: {:#?} Value: {:#?}\n", name, value));
            response.clone()
        })
        .collect()

    // "Hi".to_string()
}
