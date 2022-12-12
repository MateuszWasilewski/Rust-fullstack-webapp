use rocket::Route;

mod genes;
mod animal;

pub fn get_api_routes() -> Vec<Route> {
    routes![
        genes::genes_list,
        genes::get_phenotypes,
        animal::get_animal_list,
        animal::get_animal
    ]
}