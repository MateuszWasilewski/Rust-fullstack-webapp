use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Photo {
    pub path: String,
    pub author: Option<String>,
}
