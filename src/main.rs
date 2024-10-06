mod api;
mod config;
mod db;

use crate::config::Config;
use crate::db::Database;
use clap::Parser;
use std::time::Duration;

#[tokio::main]
async fn main() {
  dotenvy::dotenv().ok();

  let config = Config::parse();

  Database::connect(
    &config.database_url,
    config.database_pool_size,
    Duration::from_secs(5),
  )
  .await
  .unwrap();

  let app = api::routes();

  let listener =
    tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
      .await
      .unwrap();

  println!("listening on http://{}", listener.local_addr().unwrap());

  axum::serve(listener, app).await.unwrap();
}
