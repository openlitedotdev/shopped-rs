use sqlx::migrate;
use sqlx::postgres::PgPoolOptions;
use sqlx::Result;
use tracing::instrument;

use super::models::*;

#[derive(Debug, Clone)]
pub struct Database {
  pool: sqlx::PgPool,
}

impl Database {
  pub async fn connect(
    database_url: &str,
    pool_size: u32,
  ) -> anyhow::Result<Self> {
    let pool = PgPoolOptions::new()
      .max_connections(pool_size)
      .connect(database_url)
      .await?;

    if std::env::var("DATABASE_DISABLE_MIGRATIONS").is_err() {
      migrate!("./migrations")
        .run(&pool)
        .await
        .expect("ERROR: ❌ Database schema error");
    }

    println!("SUCCESS: ✅ Database connected");
    Ok(Database { pool })
  }

  #[instrument(name = "Database::create_user", skip(self, new_user), err, fields(user.name = new_user.name, user.email = new_user.email))]
  pub async fn insert_user(&self, new_user: CreateUser<'_>) -> Result<User> {
    sqlx::query_as!(
      User,
      r#"INSERT INTO users (name, email)
          VALUES ($1, $2) 
          RETURNING id, name, email, avatar_url, updated_at, created_at
        "#,
      new_user.name,
      new_user.email,
    )
    .fetch_one(&self.pool)
    .await
  }
}
