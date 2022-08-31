use std::env;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it
struct Cli {
    /// the pattern to look for
    pattern: String,
    /// the pathh to search
    path: std::path::PathBuf,
}

fn main() {
    println!("Hello, world!");
}
