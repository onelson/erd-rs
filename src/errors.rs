#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parser(#[from] pest::error::Error<crate::parser::Rule>),
}
