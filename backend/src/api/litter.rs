use db::ConnectionDB;
use rocket::serde::json::Json;
use rocket::State;

use common::litter::LitterData;

#[get("/litter-list")]
pub async fn get_litter_list(state: &State<ConnectionDB>) -> Json<Option<Vec<LitterData>>> {
    let result = db::select::litter_list(&state.pool).await.ok();

    Json(result)
}
#[post("/litter", format = "json", data = "<litter>")]
pub async fn post_litter(
    litter: Json<LitterData>,
    state: &State<ConnectionDB>,
) -> Json<Option<()>> {
    let litter = litter.into_inner();
    let result = db::insert::litter(&litter, &state.pool).await.ok();

    Json(result)
}
