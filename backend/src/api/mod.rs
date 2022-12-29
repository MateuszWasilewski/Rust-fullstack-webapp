use rocket::Route;

mod genes;
mod animal;
mod litter;

pub fn get_api_routes() -> Vec<Route> {
    routes![
        genes::genes_list,
        genes::get_phenotypes,
        genes::get_simple_phenotypes,
        animal::get_animal_list,
        animal::get_animal,
        litter::get_litter_list,
        litter::post_litter,
        animal::post_animal
    ]
}