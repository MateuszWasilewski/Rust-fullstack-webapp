use rocket::Route;

mod animal;
mod genes;
mod litter;
mod search;

pub fn get_api_routes() -> Vec<Route> {
    routes![
        genes::genes_list,
        genes::get_phenotypes,
        genes::get_simple_phenotypes,
        animal::get_animal_list,
        animal::get_animal,
        litter::get_litter_list,
        litter::post_litter,
        animal::post_animal,
        litter::get_animal_litter_list,
        search::get_search_results,
        animal::put_animal,
        animal::delete_animal
    ]
}
