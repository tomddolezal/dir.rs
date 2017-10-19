use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut dir = env::current_dir();

    if args.len() > 1 {
        dir = fs::canonicalize(PathBuf::from(&args[1]));
    }

    println!("{}", dir.unwrap().display());
}

