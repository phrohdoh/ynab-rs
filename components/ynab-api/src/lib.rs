use serde::{
    Serialize,
    Deserialize,
};

use reqwest::{
    Client as HttpClient
};

use url::Url;

use ynab_types::{
    ScheduledTransaction,
    Category,
};

const BASE_URL: &str = "https://api.youneedabudget.com/v1";

mod types;
pub use types::{
    ApiError,
    ApiErrorResponse,
    Error,
    Client,
    Response,
};

impl Client {
    pub fn new(
        bearer_token: String
    ) -> Self {
        let base_url = Url::parse(BASE_URL)
            .expect(&format!("`{}` to be a valid URL", stringify!(BASE_URL)));

        Self {
            bearer_token,
            base_url,
        }
    }

    pub fn new_with_base_url(
        bearer_token: String,
        base_url: impl Into<Url>,
    ) -> Self {
        let base_url = base_url.into();
        Self {
            bearer_token,
            base_url,
        }
    }

    pub fn get_all_scheduled_transactions(
        &self,
        budget_id: &str,
        http_client: &HttpClient,
    ) -> types::Result<Vec<ScheduledTransaction>> {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            scheduled_transactions: Vec<ScheduledTransaction>,
        }

        let url = format!(
            "{}/budgets/{}/scheduled_transactions/",
            self.base_url,
            budget_id
        );

        let user_agent = concat!(
            env!("CARGO_PKG_NAME"),
            "/",
            env!("CARGO_PKG_VERSION"),
        );

        let req = http_client.get(&url)
            .bearer_auth(&self.bearer_token)
            .header(reqwest::header::USER_AGENT, user_agent);

        let body = req.send()?.text()?;

        let resp: Response<Inner> = serde_json::from_str(&body)
            .map_err(|e| {
                let err: ApiErrorResponse = serde_json::from_str(&body)
                    .expect(&format!(
                        "failed to deserialize response body into `{}`, got {}: {:?}",
                        stringify!(ApiErrorResponse),
                        body,
                        e,
                    ));

                Error::Api(err.error)
            })?;

        Ok(resp.data.scheduled_transactions)
    }

    pub fn get_category_by_id(
        &self,
        budget_id: &str,
        category_id: &str,
        http_client: &HttpClient,
    ) -> types::Result<ynab_types::Category> {
        #[derive(Debug, Clone, Serialize, Deserialize)]
        struct Inner {
            pub category: Category,
        }

        let url = format!(
            "{}/budgets/{}/categories/{}",
            self.base_url,
            budget_id,
            category_id
        );

        let user_agent = concat!(
            env!("CARGO_PKG_NAME"), 
            "/", 
            env!("CARGO_PKG_VERSION"),
        );

        let req = http_client.get(&url)
            .bearer_auth(&self.bearer_token)
            .header(reqwest::header::USER_AGENT, user_agent);

        let body = req.send()?.text()?;
        let resp: Response<Inner> = serde_json::from_str(&body)
            .map_err(|e| {
                let err: ApiErrorResponse = serde_json::from_str(&body)
                    .expect(&format!(
                        "failed to deserialize response body into `{}`, got {}: {:?}",
                        stringify!(ApiErrorResponse),
                        body,
                        e,
                    ));

                Error::Api(err.error)
            })?;

        Ok(resp.data.category)
    }
}