use serde::Deserialize;

#[derive(Deserialize, Clone, PartialEq)]
pub struct Article {
  pub id: usize,
  pub title: String,
  pub content: String,
  pub published_at: String
}
