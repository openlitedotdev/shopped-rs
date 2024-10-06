mod users;

use axum::{routing::get, Router};

async fn healt() -> &'static str {
  "Healt server"
}

async fn welcome() -> &'static str {
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
