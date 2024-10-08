use axum::response::{self, IntoResponse};
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

use crate::db::{CreateUser, Database, User};

#[derive(Debug, Deserialize, Serialize)]
struct ServerResponse {
  users: Vec<User>,
}

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

  let server_response = ServerResponse { users };

  println!("{:?}", create_user);
  response::Json(server_response).into_response()
}

async fn login() -> &'static str {
  "Login in"
}
