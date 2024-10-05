use clap::Parser;

#[derive(Parser)]
pub struct Config {
  #[clap(long = "port", env = "PORT", default_value = "8001")]
  pub port: u16,

  #[clap(
    long = "database_url",
    env = "DATABASE_URL",
    default_value = "postgres://user:password@localhost/registry"
  )]
  /// The URL to use to connect to the database.
  pub database_url: String,

  #[clap(long = "database_pool_size", default_value = "3")]
  /// The size of the database connection pool.
  pub database_pool_size: u32,
}

impl std::fmt::Debug for Config {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Config")
      .field("port", &self.port)
      .field("database_url", &"***")
      .finish()
  }
}
