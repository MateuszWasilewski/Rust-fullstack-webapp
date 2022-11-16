
pub struct Photo {
    path: String,
    author: Option<String>,
    place: Option<String>
}

impl Photo {
    pub fn new() -> Photo{
        Photo {
            path: "/path/".to_owned(),
            author: None,
            place: None
        }
    }
}
