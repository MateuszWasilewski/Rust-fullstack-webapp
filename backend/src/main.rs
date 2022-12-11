
#[macro_use] 
extern crate rocket;
extern crate dotenv_codegen;

use std::sync::Mutex;
use sqlx::postgres::PgPoolOptions;
use dotenv_codegen::dotenv;

mod web;
mod state;
mod api;

static DB_URL: &str = dotenv!("DATABASE_URL");

#[launch]
async fn rocket() -> _ {
    let pool = PgPoolOptions::new()
        .max_connections(8)
        .connect(DB_URL).await
        .unwrap();
    sqlx::migrate!("../db/migrations")
        .run(&pool)
        .await
        .unwrap();

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