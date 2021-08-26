use std::env;
use std::process;
use semver_parser::SemVer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Not enough arguments!");
        eprintln!("Usage: {} VERSION", args[0]);
        process::exit(1);
    }

    let sv = SemVer::new(&args[1]).unwrap_or_else(|err| {
        eprintln!("Unable to parse semver: {}", err);
        process::exit(1);
    });
}
