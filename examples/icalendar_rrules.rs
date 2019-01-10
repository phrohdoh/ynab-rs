use std::env;

fn main() {
    let token = env::var("TOKEN").expect("Environment variable `TOKEN` must be set");
    let budget_id = env::var("BUDGET_ID").expect("Environment variable `BUDGET_ID` must be set");

    let http = reqwest::Client::new();
    let client = ynab::Client::new(token, budget_id);
    let stxns = match client.get_all_scheduled_transactions(&http) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("{:?}", e);
            return;
        }
    };

    for stxn in stxns {
        let rrule = stxn.frequency.as_rfc5545_rule().map(|r| r.to_string()).unwrap_or("<none>".into());
        println!("{}: freq={:?}\trrule={}", stxn.id, stxn.frequency, rrule);
    }
}