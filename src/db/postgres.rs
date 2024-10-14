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
    acquire_timeout: std::time::Duration,
  ) -> anyhow::Result<Self> {
    let pool = PgPoolOptions::new()
      .max_connections(pool_size)
      .acquire_timeout(acquire_timeout)
      .connect(database_url)
      .await?;

    if std::env::var("DATABASE_DISABLE_MIGRATIONS").is_err() {
      migrate!("./migrations")
        .run(&pool)
        .await
        .expect("ERROR: ❌ Database schema error");
    }

    tracing::info!("Connection to PostgreSQL successfully established.");
    println!("SUCCESS: ✅ Database connected");
    Ok(Database { pool })
  }

  #[instrument(name = "Database::create_user", skip(self, new_user), err, fields(new_user.name = new_user.name, user.email = new_user.email, new_user.avatar_url))]
  pub async fn insert_user(&self, new_user: CreateUser) -> Result<User> {
    sqlx::query_as!(
      User,
      r#"INSERT INTO users (name, email, avatar_url)
          VALUES ($1, $2, $3)
          RETURNING id, name, email, avatar_url, updated_at, created_at
        "#,
      new_user.name,
      new_user.email,
      new_user.avatar_url,
    )
    .fetch_one(&self.pool)
    .await
  }

  #[instrument(name = "Database::get_users", skip(self), err)]
  pub async fn get_users(&self) -> Result<Vec<User>> {
    sqlx::query_as!(User, r#"SELECT * FROM users"#)
      .fetch_all(&self.pool)
      .await
  }

  #[instrument(name = "Database::get_user_by_email", skip(self), err)]
  pub async fn get_user_by_email(&self, email: String) -> Result<User> {
    sqlx::query_as!(User, r#"SELECT * from users WHERE email LIKE $1"#, email,)
      .fetch_one(&self.pool)
      .await
  }
}
