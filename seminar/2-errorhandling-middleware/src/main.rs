use std::time::Duration;

use axum::{error_handling::HandleErrorLayer, http::StatusCode, routing::get, Router};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route(
            "/",
            get(|| async {
                println!("Executing get handler");
                tokio::time::sleep(Duration::from_secs(10)).await;
                "Hello World"
            }),
        )
        .layer(
            ServiceBuilder::new()
                // `timeout` will produce an error if the handler takes
                // too long so we must handle those
                .layer(HandleErrorLayer::new(handle_timeout_error))
                .timeout(Duration::from_secs(1)),
        );

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handle_timeout_error(err: axum::BoxError) -> (StatusCode,  &'static str) {
    if err.is::<tower::timeout::error::Elapsed>() {
        return (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            "handler this request took to long"
        )
    }
    (axum::http::StatusCode::OK, "")
}
