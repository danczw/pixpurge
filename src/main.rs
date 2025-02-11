use clap::Parser;
use glob::{glob_with, MatchOptions};
use std::fs::File;
use std::path::PathBuf;

mod purge;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long)]
    time: bool,
}

fn main() {
    let cli = Cli::parse();

    let path_in = cli.path.as_path().to_str().unwrap().trim_end_matches("/");
    let pattern = "/**/*.png";
    let traverse_pattern = format!("{path_in}{pattern}");

    let options = MatchOptions {
        case_sensitive: false,
        require_literal_separator: false,
        require_literal_leading_dot: false,
    };

    for entry in glob_with(traverse_pattern.as_str(), options).unwrap() {
        match entry {
            Ok(path) => {
                println!("{:#?}", path); // TODO: remove
                let mut file = File::options().write(true).open(path).unwrap();

                if cli.time {
                    purge::time(&mut file);
                }
            }
            Err(e) => println!("Error reading {:#?}", e),
        }
    }
}
