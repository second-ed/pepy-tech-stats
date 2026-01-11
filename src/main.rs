use clap::Parser;
use pepy_tech_stats::core::{adapters::get_real_adapter, run};

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
    let projects = vec![
        "class-inspector",
        "danom",
        "headline",
        "io-adapters",
        "readme-update",
        "repo-mapper",
        "repo-mapper-rs",
        "spaghettree",
    ]
    .into_iter()
    .map(|elem| elem.to_string())
    .collect();

    if let Err(err) = run(&mut adapter, projects, args.api_key) {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
