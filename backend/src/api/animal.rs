use anyhow::Result;
use db::ConnectionDB;
use rocket::serde::json::Json;
use rocket::State;

use common::{AnimalData, AnimalFull};

async fn fetch_animal(id: &str, state: &State<ConnectionDB>) -> Result<AnimalFull> {
    let mut animal = db::select::animal(id, &state.pool).await?;
    let animal_photos = db::select::photos_for_animal(id, &state.pool).await?;

    animal.add_photos(animal_photos);
    if let Some(litter) = &animal.litter {
        let litter_photos = db::select::photos_for_litter(litter, &state.pool).await?;
        animal.add_photos(litter_photos);
    }
    Ok(animal)
}

#[get("/animal-list")]
pub async fn get_animal_list(state: &State<ConnectionDB>) -> Json<Option<Vec<AnimalData>>> {
    let result = db::select::all_animal(&state.pool).await.ok();

    Json(result)
}

#[get("/animal/<id>")]
pub async fn get_animal(id: &str, state: &State<ConnectionDB>) -> Json<Option<AnimalFull>> {
    let animal = fetch_animal(id, state).await.ok();

    Json(animal)
}

#[post("/animal", format = "json", data = "<animal>")]
pub async fn post_animal(
    animal: Json<AnimalData>,
    state: &State<ConnectionDB>,
) -> Json<Option<()>> {
    let animal = animal.into_inner();
    let result = db::insert::animal(&animal, &state.pool).await.ok();

    Json(result)
}
