
#[derive(PartialEq)]
pub struct Photo {
    pub path: String,
    pub author: Option<String>,
}

impl Photo {
    pub fn new() -> Photo{
        Photo {
            path: "/path/".to_owned(),
            author: None,
        }
    }
}
