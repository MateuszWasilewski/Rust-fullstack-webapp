use anyhow::Result;
use db::ConnectionDB;
use rocket::futures::future;
use rocket::{serde::json::Json};
use rocket::State;

use common::{AnimalData, AnimalFull};

async fn fetch_animal(id: &str, state: &State<ConnectionDB>) -> Result<AnimalFull> {
    let get_animal = db::select::animal(id, &state.pool);
    let get_animal_photos = db::select::photos_for_animal(id, &state.pool);
    let get_phenotypes = db::select::genes_for_animal(id, &state.pool);
    let result = future::join3(get_animal, get_animal_photos, get_phenotypes).await;


    let mut animal = result.0?;
    let animal_photos = result.1?;
    let phenotypes = result.2?;

    animal.add_photos(animal_photos);
    if let Some(litter) = &animal.litter {
        let litter_photos = db::select::photos_for_litter(litter, &state.pool).await?;
        animal.add_photos(litter_photos);
    }
    animal.genes = phenotypes;
    Ok(animal)
}

#[get("/animal-list")]
pub async fn get_animal_list(state: &State<ConnectionDB>) -> Json<Option<Vec<AnimalData>>> {
    let result = db::select::all_animal(&state.pool).await.ok();

    Json(result)
}

#[get("/animal/<id>")]
pub async fn get_animal(id: &str, state: &State<ConnectionDB>) -> Option<Json<AnimalFull>> {
    let animal = fetch_animal(id, state).await.ok()?;

    Some(Json(animal))
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
