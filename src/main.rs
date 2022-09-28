use clap::Parser;
use std::fs::File;
use std::io::Error;
use std::io::{BufRead, BufReader};

/// Search for a pattern in a file and display the lines that contain it
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    pattern: String,
    /// The path to search
    #[clap(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn main() -> Result<(), Error> {
    let args = Cli::parse();
    let f = File::open(&args.path)?;
    let reader = BufReader::new(f);
    
    for line in reader.lines() {
        match line {
            Ok(line) => {
                if line.contains(&args.pattern) {
                    println!("{}", line)
                }
            }
            Err(err) => panic!("{}", err),
        };
    }
    // let content = std::fs::read_to_string(&args.path)?;
    

    // println!("file content: {}", content);

    Ok(())
}
