use axum::{
    Router,
    routing::get,
};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
    .nest_service("/",ServeDir::new("static"));
    println!("server done");

    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener,app).await.unwrap();
}
