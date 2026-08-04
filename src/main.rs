use std::path::Path;

use clap::Parser;
use pepy_tech_stats::core::{
    adapters::{get_real_adapter, Adapter, FileType, IoValue},
    run,
};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// API Key
    #[arg(long)]
    api_key: String,
}

fn main() {
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

    if let Err(err) = run(&mut adapter, &projects, args.api_key) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
