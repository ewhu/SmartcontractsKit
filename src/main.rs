// src/main.rs
/*
 * Main executable for SmartcontractsKit
 */

use clap::Parser;
use smartcontractskit::{Result, run};

#[derive(Parser)]
#[command(version, about = SmartcontractsKit - A Rust implementation)]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
