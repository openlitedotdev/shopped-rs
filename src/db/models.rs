use chrono::DateTime;
use chrono::Utc;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
  pub id: Uuid,
  pub name: String,
  pub email: String,
  pub updated_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
  pub avatar_url: Option<String>,
}

pub struct CreateUser<'s> {
  pub name: &'s str,
  pub email: &'s str,
}
