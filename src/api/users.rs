use axum::http::StatusCode;
use axum::response::{self, IntoResponse};
use axum::routing::post;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

use crate::db::{CreateUser, Database};

#[derive(Debug, Deserialize, Serialize)]
struct ServerResponse<T> {
  status: u16,
  msg: String,
  data: Option<T>,
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
  Json(mut create_user): Json<CreateUser>,
) -> impl IntoResponse {
  if create_user.name.is_empty() || create_user.email.is_empty() {
    return response::Json(ServerResponse::<()> {
      status: StatusCode::BAD_REQUEST.as_u16(),
      msg: format!("{:?} cannot be empty", "{ name, email }"),
      data: None,
    })
    .into_response();
  }

  if create_user.name.len() < 3 {
    return response::Json(ServerResponse::<()> {
      status: StatusCode::BAD_REQUEST.as_u16(),
      msg: "The value of the name property must be greater than 3".to_string(),
      data: None,
    })
    .into_response();
  }

  if !create_user.email.contains("@") {
    return response::Json(ServerResponse::<()> {
      status: StatusCode::BAD_REQUEST.as_u16(),
      msg: "The value of the email property must have an @".to_string(),
      data: None,
    })
    .into_response();
  }

  if create_user.avatar_url.is_none() {
    create_user.avatar_url = None;
  }

  let user_exist = db.get_user_by_email(create_user.clone().email).await;

  if user_exist.is_ok() {
    return response::Json(ServerResponse::<()> {
      status: StatusCode::CONFLICT.as_u16(),
      msg: "The user you tried to create already exists".to_string(),
      data: None,
    })
    .into_response();
  }

  let user = db.insert_user(create_user.clone()).await.unwrap();

  let server_response = ServerResponse {
    status: StatusCode::CREATED.as_u16(),
    msg: "User was created successfully".to_string(),
    data: Some(user.clone()),
  };

  return response::Json(server_response).into_response();
}

async fn login() -> &'static str {
  "Login in"
}
