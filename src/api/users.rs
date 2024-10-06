use axum::routing::post;
use axum::Router;

pub fn users() -> Router {
  Router::new().nest(
    "/users",
    Router::new()
      .route("/register", post(register))
      .route("/login", post(login)),
  )
}

async fn register() -> &'static str {
  "Register user"
}

async fn login() -> &'static str {
  "Login in"
}
