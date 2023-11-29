use axum::{
    extract::Extension,
    http::{Request, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response},
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(handler))
        .route_layer(middleware::from_fn(extract_auth));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn handler(Extension(auth_header_value): Extension<AuthHeaderValue>) -> Response {
    println!("{:?}", auth_header_value);
    "Hello, World!!".into_response()
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
struct AuthHeaderValue{
  pub value : String
} 

async fn extract_auth<B>(mut req: Request<B>, next: Next<B>) -> Result<Response, StatusCode> {
    let auth_value = req
        .headers()
        .get(axum::http::header::AUTHORIZATION);
    if let Some(auth_value) = auth_value {
        let auth_header_value = AuthHeaderValue { value : auth_value.to_str().unwrap().to_owned() };
        req.extensions_mut().insert(auth_header_value);

        return Ok(next.run(req).await);
    }

    Err(StatusCode::UNAUTHORIZED)
}
