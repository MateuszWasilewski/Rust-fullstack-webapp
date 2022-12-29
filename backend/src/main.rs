
#[macro_use] 
extern crate rocket;

use anyhow::Result;

mod web;
mod api;

use db::ConnectionDB;

#[rocket::main]
async fn main() -> Result<()> {
    let pool = db::connect_db().await?;
    let db_state = ConnectionDB {pool};
    csv_reader::run().await;

    let _rocket = rocket::build()
        .mount("/", web::get_routes())
        .mount("/api", api::get_api_routes())
        .manage(db_state)
        .launch()
        .await?;

    Ok(())
}