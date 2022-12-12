
#[macro_use] 
extern crate rocket;
extern crate dotenv_codegen;

use std::sync::Mutex;
use anyhow::Result;

mod web;
mod state;
mod api;
mod db;



#[rocket::main]
async fn main() -> Result<()> {
    let _pool = db::connect_db().await?;

    let mut app_state = state::GlobalState {
        counter: 0,
        global_counter: Mutex::new(0)
    };

    app_state.counter += 1;

    let _rocket = rocket::build()
        .mount("/", web::get_routes())
        .mount("/api", api::get_api_routes())
        .mount("/counter", routes![state::get_counter])
        .manage(app_state)
        .launch()
        .await?;

    Ok(())
}