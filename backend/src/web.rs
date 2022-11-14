use std::io;
use rocket::fs::NamedFile;
use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        any_file
    ]
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/dist/index.html").await
}

#[get("/<file>")]
async fn any_file(file: &str) -> io::Result<NamedFile> {
    let path = format!("{}{}", "frontend/dist/", file);
    NamedFile::open(path.as_str()).await
}