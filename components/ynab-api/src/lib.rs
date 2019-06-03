use reqwest::Client as HttpClient;
use url::Url;

const BASE_URL: &str = "https://api.youneedabudget.com/v1";

mod types;
pub use types::{
    AllScheduledTransactions,
    ApiError,
    Client,
    Response,
};

impl Client {
    pub fn new(
        bearer_token: String
    ) -> Self {
        let base_url = Url::parse(BASE_URL)
            .expect(format!("`{}` to be a valid URL", stringify!(BASE_URL)));

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
    ) -> types::Result<AllScheduledTransactions> {
        let url = format!(
            "{}/budgets/{}/scheduled_transactions/",
            self.base_url,
            budget_id
        );

        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let req = http_client.get(&url)
            .bearer_auth(&self.bearer_token)
            .header(reqwest::header::USER_AGENT, user_agent);

        let resp = {
            let body = req.send()?.text()?;
            let resp: Response<AllScheduledTransactions> = match serde_json::from_str(&body) {
                Ok(v) => v,
                Err(e) => // TODO: failure to deserialize response
            }
        };
    }
}