use serde::{Serialize, Deserialize};

use frontend_routing::Routes;

#[derive(Serialize, Deserialize, Clone, PartialEq)]
pub struct SearchResult {
  pub name: String,
  pub route: Routes,
  pub description: String
}