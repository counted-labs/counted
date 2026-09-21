use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ApiState<T> {
    None,
    Loading,
    Success(T),
    Error(ApiError),
}

#[derive(Debug, Clone, PartialEq)]
pub struct ApiError(pub String);

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
