use sqlx::migrate;
use sqlx::postgres::PgPoolOptions;

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
}
