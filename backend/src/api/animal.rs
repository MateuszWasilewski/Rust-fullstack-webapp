use anyhow::Result;
use db::ConnectionDB;
use rocket::futures::future;
use rocket::serde::json::Json;
use rocket::State;

use common::{AnimalData, AnimalFull};

async fn fetch_animal(id: &str, state: &State<ConnectionDB>) -> Result<AnimalFull> {
    let get_animal = db::select::animal(id, &state);
    let get_animal_photos = db::select::photos_for_animal(id, &state);
    let get_phenotypes = db::select::genes_for_animal(id, &state);
    let get_ancestry = plotter::generate_ancestry(id, &state);
    let result = future::join4(get_animal, get_animal_photos, get_phenotypes, get_ancestry).await;

    let mut animal = result.0?;
    let animal_photos = result.1?;
    let phenotypes = result.2?;
    let ancestry = result.3;

    if let Ok(ancestry) = ancestry {
        animal.add_photo(ancestry);
    }

    animal.add_photos(animal_photos);
    if let Some(litter) = &animal.litter {
        let litter_photos = db::select::photos_for_litter(litter, &state).await?;
        animal.add_photos(litter_photos);
    }
    animal.genes = phenotypes;
    Ok(animal)
}

#[get("/animal-list")]
pub async fn get_animal_list(state: &State<ConnectionDB>) -> Option<Json<Vec<AnimalData>>> {
    let result = db::select::all_animal(&state).await.ok()?;

    Some(Json(result))
}

#[get("/animal/<id>")]
pub async fn get_animal(id: &str, state: &State<ConnectionDB>) -> Option<Json<AnimalFull>> {
    let animal = fetch_animal(id, state).await.ok()?;

    Some(Json(animal))
}

#[post("/animal/<_id>", format = "json", data = "<animal>")]
pub async fn post_animal(
    _id: &str,
    animal: Json<AnimalData>,
    state: &State<ConnectionDB>,
) -> Option<()> {
    let animal = animal.into_inner();
    let result = db::insert::animal(&animal, &state).await.ok();

    result
}

#[put("/animal/<_id>", format = "json", data = "<animal>")]
pub async fn put_animal(
    _id: &str,
    animal: Json<AnimalData>,
    state: &State<ConnectionDB>,
) -> Option<()> {
    let animal = animal.into_inner();
    let result = db::update::animal(&animal, state).await.ok();

    result
}

#[delete("/animal/<id>")]
pub async fn delete_animal(id: &str, state: &State<ConnectionDB>) -> Option<()> {
    let result = db::delete::animal(id, &state).await.ok();

    result
}
