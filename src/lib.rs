#[macro_use]
extern crate pest_derive;

mod errors;
pub mod models;
pub mod parser;

pub use errors::Error;
pub type Result<T> = std::result::Result<T, Error>;
