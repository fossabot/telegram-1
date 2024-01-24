use serde::Deserialize;

#[derive(Debug)]
enum Response<T> {
    Ok(ResponseSuccess<T>),
    Err(ResponseError),
}

#[derive(Debug, Deserialize)]
pub struct ResponseSuccess<T> {
    ok: bool,
    result: T,
    description: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseError {
    ok: bool,
    error_code: u64,
    description: String,
    parameters: Option<ResponseParameters>,
}

#[derive(Debug, Deserialize)]
pub struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}