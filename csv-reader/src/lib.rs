#![allow(unused_variables, dead_code, non_snake_case)]

mod animals;
mod phenotypes;
mod photos;

pub async fn run() {
    phenotypes::run_phenotypes().await;
    animals::run_animals().await;
    photos::add_photos().await;
}
