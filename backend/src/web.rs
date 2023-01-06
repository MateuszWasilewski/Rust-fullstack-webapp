use rocket::fs::NamedFile;
use rocket::Route;
use std::io;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        animal_list_index,
        phenotypes,
        animal_page,
        any_page_file,
        get_photo,
        litters,
        add_data_page
    ]
}

#[get("/<_..>", rank = 0)]
async fn index() -> io::Result<NamedFile> {
    NamedFile::open("frontend/dist/index.html").await
}

#[get("/animal-list")]
async fn animal_list_index() -> io::Result<NamedFile> {
    index().await
}

#[get("/phenotypes")]
async fn phenotypes() -> io::Result<NamedFile> {
    index().await
}

#[get("/animal/<_animal_id>")]
async fn animal_page(_animal_id: &str) -> io::Result<NamedFile> {
    index().await
}

#[get("/litter-list")]
async fn litters() -> io::Result<NamedFile> {
    index().await
}

#[get("/add/<_..>")]
async fn add_data_page() -> io::Result<NamedFile> {
    index().await
}

#[get("/<file>")]
async fn any_page_file(file: &str) -> io::Result<NamedFile> {
    let path = format!("frontend/dist/{}", file);
    NamedFile::open(path.as_str()).await
}

#[get("/photo/<file>")]
async fn get_photo(file: &str) -> io::Result<NamedFile> {
    let path = format!("photos/{}", file);
    NamedFile::open(path.as_str()).await
}
