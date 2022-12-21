use rocket::serde::json::Json;
use rocket::State;
use db::ConnectionDB;

use common::{AnimalFull, AnimalData};

#[get("/animal-list")]
pub async fn get_animal_list(state: &State<ConnectionDB>) -> Json<Option<Vec<AnimalData>>> {
    let result = db::select::all_animal(&state.pool).await.ok();

    Json(result)
}

#[get("/animal/<id>")]
pub async fn get_animal(id: &str, state: &State<ConnectionDB>) -> Json<Option<AnimalFull>> {
    let animal = db::select::animal(id, &state.pool).await.ok();
    
    Json(animal)
}

#[post("/animal", format = "json", data = "<animal>")]
pub async fn post_animal(animal: Json<AnimalData>, state: &State<ConnectionDB>) -> Json<Option<()>>{
    let animal = animal.into_inner();
    let result = db::insert::animal(&animal, &state.pool).await.ok();
    
    Json(result)
}