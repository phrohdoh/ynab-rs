use serde_derive::Deserialize;

#[derive(Deserialize, Debug)]
pub struct ResponseWrapper<T> {
    pub data: T,
}

#[derive(Deserialize, Debug)]
pub struct ApiErrorResponse {
    pub error: ApiError
}

#[derive(Deserialize, Debug)]
pub struct ApiError {
    pub id: String,
    pub name: String,
    pub detail: String,
}

#[derive(Deserialize, Debug)]
pub struct AllScheduledTransactionsResponse {
    pub scheduled_transactions: Vec<ScheduledTransaction>,
}

#[derive(Deserialize, Debug)]
pub struct ScheduledTransaction {
    pub id: String,
    pub date_first: String,
    pub date_next: String,
    pub frequency: String,
    pub amount: f64,
    pub payee_id: String,
    pub payee_name: String,
    pub category_id: String,
    pub category_name: String,
}

pub struct Client {
    bearer_token: String,
    budget_id: String,
    client: reqwest::Client,
}

impl Client {
    const BASE_URL: &'static str = "https://api.youneedabudget.com/v1";

    pub fn new(bearer_token: String, budget_id: String) -> Self {
        Self {
            bearer_token,
            budget_id,
            client: reqwest::Client::new(),
        }
    }

    pub fn get_all_scheduled_transactions(&self) -> Result<Vec<ScheduledTransaction>, ApiError> {
        let url = format!("{}/budgets/{}/scheduled_transactions/", Self::BASE_URL, self.budget_id);
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let req = self.client.get(&url)
            .bearer_auth(&self.bearer_token)
            .header(reqwest::header::USER_AGENT, user_agent);

        let resp = {
            let body = req.send().unwrap().text().unwrap();
            let wrapper: ResponseWrapper<AllScheduledTransactionsResponse> = match serde_json::from_str(&body) {
                Ok(v) => v,
                Err(_) => {
                    let err: ApiErrorResponse = serde_json::from_str(&body).unwrap();
                    return Err(err.error);
                }
            };

            wrapper.data.scheduled_transactions
        };

        Ok(resp)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
