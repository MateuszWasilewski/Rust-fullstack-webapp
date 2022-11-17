use std::io;
use rocket::fs::NamedFile;
use rocket::Route;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        animal_list_index,
        any_file
    ]
}

#[get("/")]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/dist/index.html").await
}

#[get("/animal-list")]
async fn animal_list_index() -> io::Result<NamedFile> {
    index().await
}

#[get("/<file>")]
async fn any_file(file: &str) -> io::Result<NamedFile> {
    let path = format!("{}{}", "frontend/dist/", file);
    NamedFile::open(path.as_str()).await
}