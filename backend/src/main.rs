#[macro_use] extern crate rocket;

mod web;

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", web::get_routes())
    //.mount("/public", FileServer::from("frontend/dist/"))
}