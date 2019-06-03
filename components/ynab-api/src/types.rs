use url::Url;

use serde::{
    Serialize,
    Deserialize,
};

use ynab_types::ScheduledTransaction;

pub type Result<T> = std::result::Result<Response<T>, ApiError>;

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

#[derive(Debug)]
pub enum Error {
    Http(reqwest::Error),
    Api(ApiError),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllScheduledTransactions {
    pub scheduled_transactions: Vec<ScheduledTransaction>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Client {
    pub bearer_token: String,
    pub base_url: Url,
}