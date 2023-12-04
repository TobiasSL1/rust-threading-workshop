use axum::{
    response::{IntoResponse},
    routing::get,
    Router, Json,
};
use serde::Serialize;
use utoipa::{
  OpenApi, ToSchema
};

use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(paths(get_value), components(schemas(HelloData)))]
struct ApiDoc;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/get", get(get_value))
        ;
    let app = app.merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", ApiDoc::openapi()));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize, ToSchema)]
struct HelloData {
  value: String
}

#[utoipa::path(
    get,
    path = "/get",
    responses(
        (status = 200, description = "get the data", body = [HelloData])
    )
)]
async fn get_value() -> impl IntoResponse {
    Json(HelloData { value :"Hello, World!".to_owned() })
}
