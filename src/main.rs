pub mod system;

use std::path::PathBuf;

use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Filename to load
    file: Option<PathBuf>,
}



fn main() {
    let args = Args::parse();
    if let Some(path) = args.file.as_deref() {
        println!("Loading: {}", path.display());
    }
}