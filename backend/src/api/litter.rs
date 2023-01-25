use common::AnimalData;
use db::ConnectionDB;
use rocket::serde::json::Json;
use rocket::State;

use common::litter::LitterData;

#[get("/litter-list")]
pub async fn get_litter_list(state: &State<ConnectionDB>) -> Json<Option<Vec<LitterData>>> {
    let result = db::select::litter_list(&state).await.ok();

    Json(result)
}
#[post("/litter", format = "json", data = "<litter>")]
pub async fn post_litter(
    litter: Json<LitterData>,
    state: &State<ConnectionDB>,
) -> Option<Json<()>> {
    let litter = litter.into_inner();
    let result = db::insert::litter(&litter, &state).await.ok()?;

    Some(Json(result))
}

#[get("/animals-in-litter/<id>")]
pub async fn get_animal_litter_list(
    id: &str,
    state: &State<ConnectionDB>,
) -> Json<Option<Vec<AnimalData>>> {
    let animals = db::select::animals_in_litter(id, &state).await.ok();

    Json(animals)
}

#[delete("/litter/<id>")]
pub async fn delete_litter(id: &str, state: &State<ConnectionDB>) -> Option<Json<()>> {
    let result = db::delete::litter(id, state).await.ok()?;

    Some(Json(result))
}
