#[macro_use]
extern crate rocket;

use anyhow::Result;

mod api;
mod web;

use rocket::fs::FileServer;

#[rocket::main]
async fn main() -> Result<()> {
    let db_connection = db::connect_db().await?;
    csv_reader::run().await;

    let _rocket = rocket::build()
        .mount("/", web::get_routes())
        .mount("/api", api::get_api_routes())
        .mount("/public", FileServer::from("public"))
        .manage(db_connection)
        .launch()
        .await?;

    Ok(())
}
