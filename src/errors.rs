#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Parser(#[from] pest::error::Error<crate::parser::Rule>),
    #[error(transparent)]
    InvalidInt(#[from] std::num::ParseIntError),
    #[error(transparent)]
    InvalidFloat(#[from] std::num::ParseFloatError),
    #[error("Unknown formatting option: `{0}`")]
    UnknownFormatOption(String),
}
