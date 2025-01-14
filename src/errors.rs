use crate::api::responses::error::ResponseError;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
    Request(reqwest::Error),
    Response(ResponseError),
    Decode(serde_json::error::Error),
    Debug,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Request(error) => write!(f, "Request Error: {:#?}", error),
            Error::Response(error) => write!(f, "Response Error: {:#?}", error),
            Error::Decode(error) => write!(f, "Decode Error {:#?}", error),
            _ => write!(f, "Debug Error!"),
        }
    }
}
