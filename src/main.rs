use std::path::Path;

use clap::Parser;
use pepy_tech_stats::core::{
    adapters::{get_real_adapter, Adapter, FileType, IoValue, ReqwestAdapter},
    run,
};
use reqwest::Client;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// API Key
    #[arg(long)]
    api_key: String,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut adapter = get_real_adapter();

    let projects = match adapter.read(Path::new("./config/projects.txt"), FileType::Str) {
        Ok(IoValue::Str(value)) => Some(value),
        Ok(IoValue::Json(_)) => unreachable!(),
        Err(_) => None,
    }
    .unwrap_or_default()
    .trim()
    .lines()
    .map(str::to_string)
    .collect::<Vec<String>>();

    let client = ReqwestAdapter::new(Client::new());

    if let Err(err) = run(&mut adapter, &client, &projects, args.api_key).await {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
