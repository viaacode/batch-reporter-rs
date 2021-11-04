//
use std::io::Read;
//
use cli_table::{format::Justify, print_stdout, Cell, Style, Table};
use dotenv::dotenv;
use envy;
use reqwest::blocking;
use serde::Deserialize;
use structopt::StructOpt;
//
use batch_reporter_rs::MediaDataList;
use batch_reporter_rs::MediaHavenResult;

// Declare some constants
const JSON_V2: &'static str = "application/vnd.mediahaven.v2+json";

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(default = "default_env_var")]
    pub base_url: String,
    #[serde(default = "default_env_var")]
    pub mh_rest_user: String,
    #[serde(default = "default_env_var")]
    pub mh_rest_passwd: String,
}

fn default_env_var() -> String {
    String::from("default_env_var")
}

#[derive(StructOpt)]
struct Cli {
    /// The batch we want to report on.
    #[structopt(short = "b", long = "batch_name")]
    batch_name: String,
    /// Search criterion: what parameter to search by. Defaults to `batch_id`
    /// but can be any indexed field in MediaHaven. Another option for batch
    /// is `dc_identifier_localidsbatch`
    #[structopt(short = "s", long = "search_by", default_value = "batch_id")]
    search_by: String,
    /// Nr of results to return from MH's REST API. Set this to a higher value
    /// if the batch contains more than 1000 records.
    #[structopt(short = "n", long = "nr_of_results", default_value = "1000")]
    nr_of_results: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Get the cli-args
    let args = Cli::from_args();
    // Read the `.env`-file
    dotenv::dotenv().expect("Failed to read `.env` file");
    // Get our configuration from the environment. The necessary environment
    // variables can be found in the `.env` file
    let config = match envy::from_env::<Config>() {
        Ok(config) => config,
        Err(error) => panic!("{:#?}", error),
    };
    println!(
        "Searching for batch: \"{}\" (by `{}`)",
        args.batch_name, args.search_by
    );

    let url = format!(
        r##"{}/media/?q=+({}:"{}")&nrOfResults={}"##,
        config.base_url, args.search_by, args.batch_name, args.nr_of_results
    );
    //~ println!("URL: {}", url);

    let client = reqwest::blocking::Client::new();
    let mut resp = client
        .get(url)
        .header("Accept", JSON_V2)
        .basic_auth(config.mh_rest_user, Some(config.mh_rest_passwd))
        .send()?;
    let mut body = String::new();
    resp.read_to_string(&mut body)?;

    println!("Status: {}\n", resp.status());
    //~ println!("{}\n", body);

    let mediahaven_result: MediaHavenResult = serde_json::from_str(&body).unwrap();

    let on_tape: Vec<&MediaDataList> = mediahaven_result
        .media_data_list
        .iter()
        .filter(|rec| rec.internal["ArchiveStatus"] == "on_tape")
        .collect::<Vec<&MediaDataList>>();

    let on_disk: Vec<&MediaDataList> = mediahaven_result
        .media_data_list
        .iter()
        .filter(|rec| rec.internal["ArchiveStatus"] == "on_disk")
        .collect::<Vec<&MediaDataList>>();

    let in_progress: Vec<&MediaDataList> = mediahaven_result
        .media_data_list
        .iter()
        .filter(|rec| rec.internal["ArchiveStatus"] == "in_progress")
        .collect::<Vec<&MediaDataList>>();

    let failed: Vec<&MediaDataList> = mediahaven_result
        .media_data_list
        .iter()
        .filter(|rec| rec.internal["ArchiveStatus"] == "failed")
        .collect::<Vec<&MediaDataList>>();

    println!(
        "Got {} result(s) for batch: {}.",
        mediahaven_result.total_nr_of_results, args.batch_name
    );

    let table = vec![
        vec![
            "on_tape".cell(),
            on_tape.len().cell().justify(Justify::Right),
        ],
        vec![
            "on_disk".cell(),
            on_disk.len().cell().justify(Justify::Right),
        ],
        vec![
            "in_progress".cell(),
            in_progress.len().cell().justify(Justify::Right),
        ],
        vec![
            "failed".cell(),
            failed.len().cell().justify(Justify::Right)
        ],
    ]
    .table()
    .title(vec![
        "ArchiveStatus".cell().bold(true),
        "count".cell().bold(true),
    ])
    .bold(true);
    assert!(print_stdout(table).is_ok());

    Ok(())
}
