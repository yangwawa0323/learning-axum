use std::path::Display;

use askama_axum::Template;
use axum::{
    response::{Html, IntoResponse},
    routing::get,
    Router,
};

#[derive(Debug)]
pub enum Gender {
    Female,
    Male,
}

impl std::fmt::Display for Gender {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let info = match self {
            Self::Male => "Boy",
            Self::Female => "Girl",
        };
        write!(f, "{}", info)
    }
}

#[derive(Debug, Template)]
#[template(path = "sample.html")]
pub struct StudentTemplate {
    pub name: String,
    pub age: u32,
    pub sex: Gender,
}

pub fn create_routes() -> Router {
    Router::new().route("/render-template", get(render_template))
}

async fn render_template() -> impl IntoResponse {
    let student = StudentTemplate {
        name: "yangwawa".to_string(),
        age: 50,
        sex: Gender::Male,
    };

    let html = student.render().unwrap();
    Html(html)
}
