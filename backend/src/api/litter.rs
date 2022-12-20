use rocket::serde::json::Json;
use rocket::State;
use db::ConnectionDB;

use common::litter::LitterData;

#[get("/litter-list")]
pub async fn get_litter_list(state: &State<ConnectionDB>) -> Option<Json<Vec<LitterData>>> {
    let result = db::select::litter_list(&state.pool).await.ok()?;

    Some(Json(result))
}
#[post("/litter", format = "json", data = "<litter>")]
pub async fn post_litter(litter: Json<LitterData>, state: &State<ConnectionDB>) -> Option<()> {
    let litter = litter.into_inner();
    let _result = db::insert::litter(&litter, &state.pool).await.ok()?;
    Some(())
}