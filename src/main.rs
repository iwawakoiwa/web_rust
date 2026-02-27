use axum::{Router, response::Html};
use tokio::net::TcpListener;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new("static/assets"))
        .fallback(|| async {
            let html = tokio::fs::read_to_string("static/index.html")
                .await
                .unwrap_or_default();
            Html(html)
        });
    println!("server done");

    let listener = TcpListener::bind("0.0.0.0:80").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
