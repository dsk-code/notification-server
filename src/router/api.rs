mod line;

use crate::State;

use axum::{routing::get, Extension, Router};
use std::sync::Arc;

pub fn api(state: Arc<State>) -> Router {
    Router::new()
        .route(
            "/",
            get(|| async {
                println!("get /");
                "Hello, World!"
            }),
        )
        .nest_service("/line", line::router())
        .layer(Extension(state))
}
