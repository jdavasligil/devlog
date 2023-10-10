// Copyright 2023, Jaedin Davasligil, All rights reserved.

use std::{path::Path, fs::OpenOptions};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};
use chrono::{DateTime, Utc};
use csv::StringRecord;

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

fn is_clocked_in() -> Result<bool> {
    let devlog_path = Path::new(".devlog");
    let mut reader = csv::Reader::from_path(devlog_path)?;

    let mut last_record = StringRecord::default();

    for result in reader.records() {
        last_record = result?;
    }

    if last_record.into_iter().any(|field| field == "NULL".to_string()) {
        return Ok(true)
    }

    Ok(false)
}

fn clock_in() -> Result<()> {
    let devlog_path = Path::new(".devlog");
    let mut is_first_run = false;

    if !devlog_path.exists() {
        is_first_run = true;
    }

    let devlog_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(devlog_path)
    ?;

    let mut writer = csv::Writer::from_writer(devlog_file);

    if is_first_run {
        writer.write_record(&["DATE","CLOCKIN_TIME_UTC","CLOCKOUT_TIME_UTC"])?;
    }
    else if is_clocked_in()? {
        return Err(anyhow!("You are already clocked in!"));
    }

    let utc: DateTime<Utc> = Utc::now();

    writer.write_record(&[
                        utc.format("%Y-%m-%d").to_string(),
                        utc.format("%H:%M:%S").to_string(),
                        "NULL".to_string(),
    ])?;

    println!("Clocked in at {} (UTC).", utc.format("%H:%M:%S").to_string());
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
