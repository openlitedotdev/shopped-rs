use axum::response::{self, IntoResponse};
use axum::routing::post;
use axum::{Extension, Json, Router};

use crate::db::{CreateUser, Database};

pub fn users() -> Router {
  Router::new().nest(
    "/users",
    Router::new()
      .route("/register", post(register))
      .route("/login", post(login)),
  )
}

async fn register(
  Extension(db): Extension<Database>,
  Json(create_user): Json<CreateUser>,
) -> impl IntoResponse {
  let users = db.get_users().await.unwrap();

  println!("{:?}", create_user);
  response::Json(users).into_response()
}

async fn login() -> &'static str {
  "Login in"
}
