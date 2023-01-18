use serde::{Serialize, Deserialize};



#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResult {
  pub name: String,
  pub path: String,
  pub description: String
}