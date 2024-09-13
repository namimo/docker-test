use axum::{
    body::Body,
    http,
    response::IntoResponse,
    routing::{any, get},
    Router,
};
use dotenvy::dotenv;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let port = std::env::var("PORT").expect("PORT is not set");
    let addr = format!("0.0.0.0:{port}");
    let listener = TcpListener::bind(&addr)
        .await
        .expect("Unable to connect to the server");
    let app = Router::new().route("/", get(index)).route("/all", any(all));

    axum::serve(listener, app)
        .await
        .expect("Error serving application");
}

async fn index() -> impl IntoResponse {
    "Hello, World!"
}

async fn all(req: http::Request<Body>) -> impl IntoResponse {
    println!("{:#?}", req);

    http::StatusCode::OK
}
