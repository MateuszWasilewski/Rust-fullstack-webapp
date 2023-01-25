use common::{animal::Gender, SearchResult};
use db::ConnectionDB;
use rocket::{serde::json::Json, State};

use frontend_routing::Routes;

#[get("/search/<query>")]
pub async fn get_search_results(
    query: String,
    state: &State<ConnectionDB>,
) -> Json<Vec<SearchResult>> {
    let animals = db::select::animals_for_query(&query, &state).await;
    //let litters = db::select::litter_list(&state.pool).await;

    let mut results: Vec<SearchResult> = vec![];
    if let Ok(animals) = animals {
        let mut search_animals: Vec<SearchResult> = animals
            .into_iter()
            .map(|animal| SearchResult {
                name: animal.id.clone(),
                route: Routes::GoToAnimal { id: animal.id },
                description: format!(
                    "płeć: {}, fenotyp: {}",
                    match animal.gender {
                        Gender::Male => "M",
                        Gender::Female => "F",
                    },
                    animal.fenotyp.unwrap_or("nieznany".into())
                ),
            })
            .collect();
        results.append(&mut search_animals);
    }

    Json(results)
}
