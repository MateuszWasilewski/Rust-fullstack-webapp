use std::sync::Mutex;

#[macro_use] extern crate rocket;

mod web;
mod state;
mod api;

#[launch]
fn rocket() -> _ {
    let mut app_state = state::GlobalState {
        counter: 0,
        global_counter: Mutex::new(0)
    };

    app_state.counter += 1;

    rocket::build()
        .mount("/", web::get_routes())
        .mount("/api", api::get_api_routes())
        .mount("/counter", routes![state::get_counter])
        .manage(app_state)
    //.mount("/public", FileServer::from("frontend/dist/"))
}