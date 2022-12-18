use rocket::serde::json::Json;
use rocket::State;
use crate::state::ConnectionDB;
use crate::db;

use common::Animal;

#[get("/animal-list")]
pub async fn get_animal_list(state: &State<ConnectionDB>) -> Option<Json<Vec<Animal>>> {
    let result = db::select::all_animal(&state.pool).await.ok()?;

    Some(Json(result))
}

#[get("/animal/<id>")]
pub async fn get_animal(id: &str, state: &State<ConnectionDB>) -> Option<Json<Animal>> {
    let animal = db::select::animal(id, &state.pool).await.ok()?;
    
    Some(Json(animal))
}