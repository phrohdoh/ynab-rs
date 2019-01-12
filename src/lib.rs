use serde_derive::Deserialize;
use reqwest::Client as HttpClient;

mod api_types;

#[cfg(feature = "rfc5545")]
mod ext_rfc5545;

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
    pub frequency: RecurFrequency,
    pub amount: f64,
    pub payee_id: String,
    pub payee_name: String,
    pub category_id: String,
    pub category_name: String,
    pub deleted: bool,
}

impl ScheduledTransaction {
    pub fn new_from_api_model(api_model: api_types::ScheduledTransaction) -> Result<Self, String> {
        Ok(Self {
            frequency: RecurFrequency::try_from_str(&api_model.frequency).map_err(|_| format!("Unable to parse `{}` into frequency", api_model.frequency))?,
            id: api_model.id,
            date_first: api_model.date_first,
            date_next: api_model.date_next,
            amount: api_model.amount,
            payee_id: api_model.payee_id,
            payee_name: api_model.payee_name,
            category_id: api_model.category_id,
            category_name: api_model.category_name,
            deleted: api_model.deleted,
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
#[derive(PartialEq, Clone, Debug)]
pub enum RecurFrequency {
    Never,
    Daily,
    Weekly,
    EveryOtherWeek,
    TwiceAMonth,
    Every4Weeks,
    Monthly,
    EveryOtherMonth,
    Every3Months,
    Every4Months,
    TwiceAYear,
    Yearly,
    EveryOtherYear,
}

impl RecurFrequency {
    pub fn try_from_str(s: &str) -> Result<Self, ()> {
        use self::RecurFrequency::*;

        Ok(match s {
            "never" => Never,
            "daily" => Daily,
            "weekly" => Weekly,
            "everyOtherWeek" => EveryOtherWeek,
            "twiceAMonth" => TwiceAMonth,
            "every4Weeks" => Every4Weeks,
            "monthly" => Monthly,
            "everyOtherMonth" => EveryOtherMonth,
            "every3Months" => Every3Months,
            "every4Months" => Every4Months,
            "twiceAYear" => TwiceAYear,
            "yearly" => Yearly,
            "everyOtherYear" => EveryOtherYear,
            _ => return Err(()),
        })
    }
}

pub struct Client {
    bearer_token: String,
    budget_id: String,
}

impl Client {
    const BASE_URL: &'static str = "https://api.youneedabudget.com/v1";

    pub fn new(bearer_token: String, budget_id: String) -> Self {
        Self {
            bearer_token,
            budget_id,
        }
    }

    pub fn get_all_scheduled_transactions(&self, client: &HttpClient) -> Result<Vec<ScheduledTransaction>, ApiError> {
        let url = format!("{}/budgets/{}/scheduled_transactions/", Self::BASE_URL, self.budget_id);
        let user_agent = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

        let req = client.get(&url)
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
