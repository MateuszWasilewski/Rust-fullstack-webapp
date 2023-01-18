use common::SearchResult;
use db::ConnectionDB;
use rocket::{serde::json::Json, State};

#[get("/search/<query>")]
pub async fn get_search_results(
    query: String,
    state: &State<ConnectionDB>,
) -> Json<Vec<SearchResult>> {
    let animals = db::select::animals_for_query(&query, &state.pool).await;
    //let litters = db::select::litter_list(&state.pool).await;

    let mut results: Vec<SearchResult> = vec![];
    if let Ok(animals) = animals {
      let mut search_animals: Vec<SearchResult> = animals.into_iter().map(|animal| {
        SearchResult { 
          path: format!("animal/{}", animal.id),
          name: animal.id, 
          description: "Mysz o id: ".into()
        }
      }).collect();
      results.append(&mut search_animals);
    }
    
    Json(results)
}
