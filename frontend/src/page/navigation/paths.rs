pub static HOME: &str = "/";
pub static ANIMAL_LIST: &str = "/animal_list";
pub static LITTER_LIST: &str = "litter_list";

pub fn animal(id: &str) -> String {
    format!("/animal/{}", id)
}

pub fn litter(id: &str) -> String {
    format!("/litter/{}", id)
}
