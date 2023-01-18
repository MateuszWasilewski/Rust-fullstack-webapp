pub mod animal;
mod base;
pub mod litter;
mod types;
mod request;

#[macro_use]
extern crate anyhow;

pub use request::get;