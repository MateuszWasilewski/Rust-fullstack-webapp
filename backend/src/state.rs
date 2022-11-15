use rocket::State;
use std::sync::Mutex;
pub struct GlobalState {
    pub counter: u64,
    pub global_counter: Mutex<u64>
}

#[get("/")]
pub fn get_counter(state: &State<GlobalState>) -> String {
    let var = state.global_counter.lock();
    let mut guard = var.expect("No value");
    *guard += 1;
    (*guard).to_string()
}