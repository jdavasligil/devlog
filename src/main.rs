// Copyright 2023, Jaedin Davasligil, All rights reserved.

use clap::{Parser, Subcommand};

/// Program to track developer time by clocking in and out
#[derive(Parser, Debug)]
struct Args {
    /// The command to be called
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    In,
    Out,
}

fn clock_in() {
    println!("Clock in!")
}

fn clock_out() {
    println!("Clock out!")
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::In => clock_in(),
        Commands::Out => clock_out(),
    }
}
