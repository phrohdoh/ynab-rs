use std::{
    str::{
        FromStr,
    },
    io,
};

use serde::{
    Deserialize,
};

use uuid::{
    Uuid,
};

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

        #[structopt(long = "output-format")]
        output_format: Option<OutputFormat>,
    },

    #[structopt(name = "get-category-by-id")]
    GetCategoryById {
        #[structopt(flatten)]
        bearer_token_opt: BearerTokenOpt,

        #[structopt(long = "budget-id")]
        budget_id: Uuid,

        #[structopt(long = "category-id")]
        category_id: Uuid,

        #[structopt(long = "output-format")]
        output_format: Option<OutputFormat>,
    },
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
enum OutputFormat {
    /// Rust's pretty-printed debug format
    Human,
    /// Machine-readable single-line JSON
    Json,
    /// Machine-readable pretty-printed JSON
    JsonPretty,
}

impl FromStr for OutputFormat {
    type Err = serde_json::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(serde_json::from_str(&format!("\"{}\"", s))?)
    }
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
        Command::GetAllScheduledTransactions {
            bearer_token_opt,
            budget_id,
            output_format,
        } => {
            let client = Client::new(bearer_token_opt.bearer_token);
            let res = client.get_all_scheduled_transactions(&budget_id.to_string(), &http_client);
            let stxns = res.expect("to not get an error");

            match output_format {
                Some(OutputFormat::Json) => {
                    let _ = serde_json::to_writer(io::stdout().lock(), &stxns);
                    println!();
                },
                Some(OutputFormat::JsonPretty) => {
                    let _ = serde_json::to_writer_pretty(io::stdout().lock(), &stxns);
                    println!();
                },
                Some(OutputFormat::Human) | _ => {
                    println!("{:#?}", stxns);
                },
            }
        },
        Command::GetCategoryById {
            bearer_token_opt,
            budget_id,
            category_id,
            output_format,
        } => {
            let client = Client::new(bearer_token_opt.bearer_token);
            let res = client.get_category_by_id(&budget_id.to_string(), 
                &category_id.to_string(), 
                &http_client);
            let stxns = res.expect("to not get an error");

            match output_format {
                Some(OutputFormat::Json) => {
                    let _ = serde_json::to_writer(io::stdout().lock() , &stxns);
                    println!();
                },
                Some(OutputFormat::JsonPretty) => {
                    let _ = serde_json::to_writer_pretty(io::stdout().lock() , &stxns);
                },
                Some(OutputFormat::Human) | _ => {
                    println!("{:#?}", stxns);
                },
            }
        },
    }
}