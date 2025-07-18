// src/main.rs
/*
 * Main executable for ArtificialMasterpieceGeneratorDevX
 */

use clap::Parser;
use artificialmasterpiecegeneratordevx::{Result, run};

#[derive(Parser)]
#[command(version, about = "ArtificialMasterpieceGeneratorDevX - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
