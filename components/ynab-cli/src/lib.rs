use uuid::Uuid;

use reqwest::{
    Client as HttpClient,
};

use structopt::{
    StructOpt,
};

use ynab_api::{
    Client,
};

#[derive(Debug, StructOpt)]
struct Opt {
    #[structopt(subcommand)]
    command: Command,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get-all-scheduled-transactions")]
    /// Get all scheduled transactions for a budget
    GetAllScheduledTransactions {
        #[structopt(flatten)]
        bearer_token_opt: BearerTokenOpt,

        #[structopt(long = "budget-id")]
        budget_id: Uuid,
    },
}

#[derive(Debug, StructOpt)]
struct BearerTokenOpt {
    #[structopt(long = "bearer-token")]
    bearer_token: String,
}

pub fn main() {
    let opt = Opt::from_args();
    run(opt)
}

fn run(opt: Opt) {
    let http_client = HttpClient::new();

    match opt.command {
        Command::GetAllScheduledTransactions { bearer_token_opt, budget_id } => {
            let client = Client::new(bearer_token_opt.bearer_token);
            let res = client.get_all_scheduled_transactions(&budget_id.to_string(), &http_client);
            let stxns = res.expect("to get back scheduled transactions");
            println!("{:#?}", stxns);
        },
    }
}