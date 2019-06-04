use std::{
    str::{
        FromStr,
    },
    fmt,
    io,
};

use serde::{
    Serialize,
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
    Error,
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
    /// Get a category in a budget by its ID
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
            get_all_scheduled_transactions(
                budget_id.to_string(),
                output_format,
                &Client::new(bearer_token_opt.bearer_token),
                &http_client,
            );
        },
        Command::GetCategoryById {
            bearer_token_opt,
            budget_id,
            category_id,
            output_format,
        } => {
            get_category_by_id(
                budget_id.to_string(),
                category_id.to_string(),
                output_format,
                &Client::new(bearer_token_opt.bearer_token),
                &http_client,
            );
        },
    }
}

fn get_all_scheduled_transactions(
    budget_id: impl AsRef<str>,
    output_format: Option<OutputFormat>,
    client: &Client,
    http_client: &HttpClient,
) {
    let scheduled_transactions = match client.get_all_scheduled_transactions(
        budget_id.as_ref(),
        http_client,
    ) {
        Ok(stxns) => stxns,
        Err(e) => match e {
            Error::Api(ref e) if e.is_resource_not_found() => {
                eprintln!("A resource could not be found.  Please double check the values given (such as budget-id).");
                return;
            },
            _ => unimplemented!(),
        },
    };

    output(output_format, &scheduled_transactions);
}

fn get_category_by_id(
    budget_id: impl AsRef<str>,
    category_id: impl AsRef<str>,
    output_format: Option<OutputFormat>,
    client: &Client,
    http_client: &HttpClient,
) {
    let category = match client.get_category_by_id(
        budget_id.as_ref(),
        category_id.as_ref(),
        http_client,
    ) {
        Ok(category) => category,
        Err(e) => match e {
            Error::Api(ref e) if e.is_resource_not_found() => {
                eprintln!("A resource could not be found.  Please double check the values given (such as budget-id).");
                return;
            },
            _ => unimplemented!(),
        },
    };

    output(output_format, &category);
}

fn output<T: fmt::Debug + Serialize>(fmt: Option<OutputFormat>, t: &T) {
    match fmt {
        Some(OutputFormat::Json) => {
            let _ = serde_json::to_writer(
                io::stdout().lock(),
                t,
            );
            println!();
        },
        Some(OutputFormat::JsonPretty) => {
            let _ = serde_json::to_writer_pretty(
                io::stdout().lock(),
                t,
            );
            println!();
        },
        Some(OutputFormat::Human) | None => {
            println!("{:#?}", t);
        },
    }
}