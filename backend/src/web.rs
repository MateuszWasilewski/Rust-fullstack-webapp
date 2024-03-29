use rocket::fs::NamedFile;
use rocket::Route;
use std::io;
use std::path::PathBuf;
use std::path::Path;

pub fn get_routes() -> Vec<Route> {
    routes![
        index,
        animal_list_index,
        phenotypes,
        animal_page,
        any_page_file,
        litters,
        add_data_page,
        litter_page,
        search_results,
        error,
        not_found
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

#[get("/phenotypes")]
async fn phenotypes() -> io::Result<NamedFile> {
    index().await
}

#[get("/animal/<_animal_id>")]
async fn animal_page(_animal_id: &str) -> io::Result<NamedFile> {
    index().await
}

#[get("/litter/<_litter_id>")]
async fn litter_page(_litter_id: &str) -> io::Result<NamedFile> {
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

#[get("/search/<_..>")]
async fn search_results() -> io::Result<NamedFile> {
    index().await
}

#[get("/error")]
async fn error() -> io::Result<NamedFile> {
    index().await
}

#[get("/404")]
async fn not_found() -> io::Result<NamedFile> {
    index().await
}

#[get("/<path>")]
async fn any_page_file(path: PathBuf) -> io::Result<NamedFile> {
    let path = Path::new("frontend/dist/").join(path);
    NamedFile::open(path.as_path()).await
}