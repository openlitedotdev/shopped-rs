mod api;
mod config;
mod db;

use crate::config::Config;
use crate::db::Database;
use axum::http::{HeaderValue, Method};
use axum::Extension;
use clap::Parser;
use std::time::Duration;
use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
  tracing_subscriber::fmt::init();
  dotenvy::dotenv().ok();

  let config = Config::parse();
  let cors = CorsLayer::new()
    .allow_credentials(true)
    .allow_origin("http://localhost:3000".parse::<HeaderValue>().unwrap())
    .allow_methods([Method::GET, Method::POST])
    .max_age(Duration::from_secs(3600));

  let db = Database::connect(
    &config.database_url,
    config.database_pool_size,
    Duration::from_secs(5),
  )
  .await
  .unwrap();

  let app = api::routes().layer(cors).layer(Extension(db));

  let listener =
    tokio::net::TcpListener::bind(format!("127.0.0.1:{}", config.port))
      .await
      .unwrap();

  tracing::info!("✅ Starting server...");
  println!("listening on http://{}", listener.local_addr().unwrap());

  axum::serve(listener, app).await.unwrap();
}
