use axum::{
    extract::Query,
    http::StatusCode,
    response::Html,
    routing::get,
    Router,
};
use serde::Deserialize;
use tower_http::cors::CorsLayer;
use tracing::{info, Level};

mod badge;
mod image;
mod template;
mod cache;

#[derive(Debug, Deserialize)]
struct BadgeParams {
    ver: Option<String>,
    url: Option<String>,
    txt: Option<String>,
    size: Option<u32>,
    border: Option<u32>,
    barlen: Option<String>,
    fontsize: Option<u32>,
    barradius: Option<u32>,
    scale: Option<f32>,
    fontcolor: Option<String>,
    shadow: Option<f32>,
    backcolor: Option<String>,
    anime: Option<f32>,
    way: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .init();

    let app = Router::new()
        .route("/gen", get(generate_badge))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("Server running on http://0.0.0.0:3000");
    
    axum::serve(listener, app).await.unwrap();
}

async fn generate_badge(Query(params): Query<BadgeParams>) -> Result<Html<String>, StatusCode> {
    match badge::generate_badge(params).await {
        Ok(svg) => Ok(Html(svg)),
        Err(e) => {
            eprintln!("badge error: {:?}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}