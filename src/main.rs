use axum::{self, response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;
use dotenvy::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT is not set");
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to connect to the server");
    let app = Router::new().route("/", get(index));

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}
