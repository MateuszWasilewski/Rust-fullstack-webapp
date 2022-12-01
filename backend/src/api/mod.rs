use rocket::Route;

mod genes;

pub fn get_api_routes() -> Vec<Route> {
    routes![
        genes::genes_list
    ]
}