use axum::{routing::get, Router};
use pprof_integration::frameworks::axum::pprof_profile_axum;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/debug/pprof/profile", get(pprof_profile_axum));
    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello, World!"
}
