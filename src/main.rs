mod config;
mod db;

use crate::config::Config;
use crate::db::Database;
use axum::{response::Html, routing::get, Router};
use clap::Parser;

async fn handler() -> Html<&'static str> {
  Html("<h1>Hello, World!</h1>")
}

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();
  let config = Config::parse();

  let app = Router::new().route("/", get(handler));

  let connect =
    Database::connect(&config.database_url, config.database_pool_size)
      .await
      .unwrap();

  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
    .await
    .unwrap();

  println!("listening on http://{}", listener.local_addr().unwrap());

  axum::serve(listener, app).await.unwrap();
}
