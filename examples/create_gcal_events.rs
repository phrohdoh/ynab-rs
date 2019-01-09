use std::env;
use ynab::RecurFrequency as Freq;
use easyg::cal::{Calendar, Event, EventDateTime};

fn main() {
    let token = env::var("TOKEN").expect("Environment variable `TOKEN` must be set");
    let cal_id = env::var("CAL_ID").expect("Environment variable `CAL_ID` must be set");

    let http = reqwest::Client::new();
    let mut google = easyg::Client::new(token, "".into(), "".into(), "".into());

    let freqs = [
        Freq::Never,
        Freq::Daily,
        Freq::Weekly,
        Freq::EveryOtherWeek,
        Freq::TwiceAMonth,
        Freq::Every4Weeks,
        Freq::Monthly,
        Freq::EveryOtherMonth,
        Freq::Every3Months,
        Freq::Every4Months,
        Freq::TwiceAYear,
        Freq::Yearly,
        Freq::EveryOtherYear,
    ];

    for freq in &freqs {
        let recurrence_rules = freq.as_rfc5545_rule()
            .into_iter()
            .map(|r| r.to_string())
            .collect::<Vec<_>>();

        let event = Event {
            id: uuid::Uuid::new_v4().to_simple().to_string(),
            summary: format!("{:?}", freq),
            start: EventDateTime {
                date: "2019-01-09".into(),
                iana_time_zone: "America/Chicago".into(),
            },
            end: Some(EventDateTime {
                date: "2019-01-09".into(),
                iana_time_zone: "America/Chicago".into(),
            }),
            recurrence: recurrence_rules.clone(),
            description: Some(recurrence_rules.join(" # ")),
        };

        print!("{} {:?} ... ", event.id, freq);

        if let Err(e) = google.event_insert(&http, &cal_id, &event) {
            println!("error");

            use easyg::cal::EventInsertionError::*;
            match e {
                TokenExpired => eprintln!("Token rejected. Please re-run with a new access token."),
                _ => eprintln!("Error creating `{:?}` event: {:?}", freq, e),
            }

            break;
        }

        println!("done");
    }
}
