use clap::Parser;
use glob::{glob_with, MatchOptions};
use std::fs::{self, File, FileTimes};
use std::path::PathBuf;
use std::time::SystemTime;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: PathBuf,
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
                let _dirt_data = fs::metadata(&path);
                let clean_file = File::options().write(true).open(path).unwrap();
                let times = FileTimes::new()
                    .set_accessed(SystemTime::UNIX_EPOCH)
                    .set_modified(SystemTime::UNIX_EPOCH);

                clean_file
                    .set_times(times)
                    .expect("Error updating file times");
            }
            Err(e) => println!("Error reading {:#?}", e),
        }
    }
}
