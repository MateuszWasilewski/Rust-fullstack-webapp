use common::Phenotype;
use rocket::serde::json::Json;
use rocket::State;
use db::ConnectionDB;

#[get("/genes-list")]
pub fn genes_list() -> Json<Vec<String>>{
    let result = vec![
        "ASIP1".into(),
        "ASIP2".into(),
        "TYRP1".into(),
        "TYRP2".into(),
        "TYR1".into(),
        "TYR2".into(),
        "MYO5A1".into(),
        "MYO5A2".into(),
        "OCA1".into(),
        "OCA2".into(),
        "MLPH1".into(),
        "MLPH2".into(),
        "PMEL1".into(),
        "PMEL2".into(),
        "ATP7A1".into(),
        "ATP7A2".into(),
        "KIT1".into(),
        "KIT2".into(),
        "MC1R1".into(),
        "MC1R2".into(),
        "AP3B1".into(),
        "AP3B2".into(),
        "U1".into(),
        "U2".into(),
        "RW1".into(),
        "RW2".into(),
        "SPL1".into(),
        "SPL2".into(),
        "RN1".into(),
        "RN2".into(),
        "FZ1".into(),
        "FZ2".into(),
    ];
    Json(result)
}

#[get("/phenotype-list")]
pub async fn get_phenotypes(state: &State<ConnectionDB>) -> Option<Json<Vec<Phenotype>>> {
    let phenotypes = db::select::phenotype_genes_list(&state.pool).await.ok()?;

    Some(Json(phenotypes))
}

#[get("/simple-phenotype-list")]
pub async fn get_simple_phenotypes(state: &State<ConnectionDB>) -> Option<Json<Vec<Phenotype>>> {
    let phenotypes = db::select::phenotype_list(&state.pool).await.ok()?;

    Some(Json(phenotypes))
}
