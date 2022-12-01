use rocket::serde::json::Json;


#[get("/genes-list")]
pub fn genes_list() -> Json<Vec<String>>{
    let result = vec![
        "ASIP".to_owned(),
        "ASIP2".to_owned(),
        "TYRP1".to_owned(),
        "TYRP2"	.to_owned(),
        "TYR".to_owned(),
        "TYR2".to_owned(),
        "MYO5A".to_owned(),
        "MYO5A2".to_owned(),
        "OCA2".to_owned(),
        "OCA3".to_owned(),
    ];
    Json(result)
}