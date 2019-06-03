use url::Url;

use serde::{
    Serialize,
    Deserialize,
};

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response<T> {
    pub data: T,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiErrorResponse {
    pub error: ApiError
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiError {
    pub id: String,
    pub name: String,
    pub detail: String,
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Http(e)
    }
}

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Api(ApiError),
}

#[derive(Debug, Clone)]
pub struct Client {
    pub bearer_token: String,
    pub base_url: Url,
}
