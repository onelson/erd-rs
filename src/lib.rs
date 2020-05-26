#[macro_use]
extern crate pest_derive;

pub mod er;
mod errors;
mod parser;
mod render;

pub use errors::Error;
pub type Result<T> = std::result::Result<T, Error>;
