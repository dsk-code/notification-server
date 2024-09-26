mod error;

use axum::{routing::get, Router};

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    let app = Router::new().route(
        "/",
        get(|| async {
            println!("get /");
            "Hello, World!"
        }),
    );

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8001").await?;
    println!("* start server 0.0.0.0:8001");
    axum::serve(listener, app).await?;

    Ok(())
}
