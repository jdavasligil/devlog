// Copyright 2023, Jaedin Davasligil, All rights reserved.

use std::path::Path;

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use chrono::{DateTime, Utc, Local, TimeZone};

/// Program to track developer time by clocking in and out
#[derive(Parser, Debug)]
struct Args {
    /// The command to be called
    #[clap(subcommand)]
    command: Commands,
}

/// List of available commands
#[derive(Debug, Subcommand)]
enum Commands {
    In,
    Out,
}

fn clock_in() -> Result<()> {
    let devlog_path = Path::new("DEVLOG.csv");
    let mut writer = csv::Writer::from_path(devlog_path)?;

    if !devlog_path.exists() {
        writer.write_record(&["DATE","TIME_UTC","TIME_LOCAL"])?;
    }

    let utc: DateTime<Utc> = Utc::now();
    let local: DateTime<Local> = Local::now();

    writer.write_record(&[
                        utc.format("%Y-%m-%d").to_string(),
                        utc.format("%H:%M:%S").to_string(),
                        local.format("%H:%M:%S").to_string(),
    ])?;

    println!("Clocked in at {}.", local.format("%H:%M:%S").to_string());
    Ok(())
}

fn clock_out() {
    println!("Clock out!");
}

fn main() -> Result<()> {
    let args = Args::parse();

    match args.command {
        Commands::In => clock_in()?,
        Commands::Out => clock_out(),
    }

    Ok(())
}
