use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone)]
pub struct Photo {
    pub path: String,
    pub author: Option<String>,
}

impl Photo {
    pub fn new() -> Photo {
        Photo {
            path: "/path/".to_owned(),
            author: None,
        }
    }
}
