use super::parameters::ResponseParameters;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct ResponseError {
    pub ok: bool,
    pub error_code: u64,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<ResponseParameters>,
}

impl ResponseError {
    pub fn new(body: &str) -> Self {
        serde_json::from_str::<ResponseError>(body).unwrap()
    }
}
