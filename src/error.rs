#[derive(Debug)]
pub enum Error {
    InvalidParamError
    //..
}

pub type Result<T> = std::result::Result<T, Error>;