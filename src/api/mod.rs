mod users;

use axum::{response::IntoResponse, routing::get, Router};

async fn healt() -> impl IntoResponse {
  "Healt server"
}

async fn welcome() -> impl IntoResponse {
  "Welcome server API shopped Open source"
}

fn root() -> Router {
  Router::new().route("/healt", get(healt))
}

pub fn routes() -> Router {
  Router::new()
    .nest("/api", Router::new().merge(root()).merge(users::users()))
    .route("/", get(welcome))
}
