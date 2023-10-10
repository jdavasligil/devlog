// Copyright 2023, Jaedin Davasligil, All rights reserved.

use std::{
    fs::OpenOptions,
    io::{Seek, SeekFrom, Write},
    path::Path,
};

use anyhow::{anyhow, Result};
use chrono::{DateTime, Duration, Utc};
use clap::{Parser, Subcommand};
use csv::StringRecord;
use serde::{Deserialize, Serialize};

/// Program to track developer time by clocking in and out.
#[derive(Parser, Debug)]
struct Args {
    /// The command to be called.
    #[clap(subcommand)]
    command: Commands,
}

/// List of available commands.
#[derive(Debug, Subcommand)]
enum Commands {
    In,
    Out,
    Time,
}

#[derive(Debug, Deserialize, Serialize)]
struct Record {
    date: String,
    in_time_utc: String,
    out_time_utc: String,
    delta_time_utc: String,
}

fn get_last_record(path_str: &str) -> Result<StringRecord> {
    let devlog_path = Path::new(path_str);
    let mut reader = csv::Reader::from_path(devlog_path)?;

    let mut last_record = StringRecord::default();

    for result in reader.records() {
        last_record = result?;
    }

    Ok(last_record)
}

fn is_clocked_in(path_str: &str) -> Result<bool> {
    let last_record = get_last_record(path_str)?;

    if last_record.iter().last() == Some("") {
        return Ok(true);
    }

    Ok(false)
}

fn get_last_clockin_time(last_record: StringRecord) -> Result<DateTime<Utc>> {
    let mut utc_string = String::new();
    let fmt_str = "%Y-%m-%d %H:%M:%S %z";

    utc_string.push_str(&last_record[0]);
    utc_string.push_str(" ");
    utc_string.push_str(&last_record[1]);
    utc_string.push_str(" +00:00");

    let utc_last: DateTime<Utc> =
        DateTime::parse_from_str(utc_string.as_str(), fmt_str)?.try_into()?;

    Ok(utc_last)
}

fn clock_in(path_str: &str) -> Result<()> {
    let devlog_path = Path::new(path_str);
    let mut is_first_run = false;

    if !devlog_path.exists() {
        is_first_run = true;
    }

    let devlog_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(devlog_path)?;

    let mut writer = csv::Writer::from_writer(devlog_file);

    if is_first_run {
        let header = ["date", "in_time_utc", "out_time_utc", "delta_time_utc"];
        writer.write_record(&header)?;
    } else if is_clocked_in(path_str)? {
        return Err(anyhow!("You are already clocked in."));
    }

    let utc: DateTime<Utc> = Utc::now();

    writer.write_record(&[
        utc.format("%Y-%m-%d").to_string(),
        utc.format("%H:%M:%S").to_string(),
        "".to_string(),
        "".to_string(),
    ])?;

    println!(
        "Clocked in at {} (UTC).",
        utc.format("%H:%M:%S").to_string()
    );

    Ok(())
}

fn clock_out(path_str: &str) -> Result<()> {
    let devlog_path = Path::new(path_str);

    if !devlog_path.exists() || !is_clocked_in(path_str)? {
        return Err(anyhow!("You need to clock in first."));
    }

    let mut devlog_file = OpenOptions::new()
        .write(true)
        .create(false)
        .append(false)
        .open(devlog_path)?;

    // Magic number -2: seeks to the first empty field by jumping over the ','.
    (&devlog_file).seek(SeekFrom::End(-2))?;

    let last_record = get_last_record(path_str)?;

    let utc_curr: DateTime<Utc> = Utc::now();
    let utc_last: DateTime<Utc> = get_last_clockin_time(last_record)?;
    let delta: Duration = utc_curr.signed_duration_since(utc_last);

    let mut write_str = String::new();
    write_str.push_str(utc_curr.format("%H:%M:%S").to_string().as_str());
    write_str.push_str(",");
    write_str.push_str(delta.num_seconds().to_string().as_str());
    write_str.push_str("\n");

    devlog_file.write(write_str.as_bytes())?;

    println!(
        "Clocked out at {} (UTC).",
        utc_curr.format("%H:%M:%S").to_string()
    );
    Ok(())
}

fn get_time(path_str: &str) -> Result<()> {
    let devlog_path = Path::new(path_str);

    if !devlog_path.exists() {
        return Err(anyhow!("No time record detected."));
    }

    let mut rdr = csv::Reader::from_path(devlog_path)?;
    let mut second_counter: i64 = 0;

    if is_clocked_in(path_str)? {
        let last_record = get_last_record(path_str)?;
        let utc_curr: DateTime<Utc> = Utc::now();
        let utc_last: DateTime<Utc> = get_last_clockin_time(last_record)?;
        let delta: Duration = utc_curr.signed_duration_since(utc_last);

        second_counter += delta.num_seconds();
    }

    for record in rdr.records() {
        let seconds: i64 = record?
            .iter()
            .last()
            .unwrap_or_default()
            .parse()
            .unwrap_or_default();
        second_counter += seconds;
    }

    let seconds = second_counter % 60;
    let minutes = (second_counter / 60) % 60;
    let hours = (second_counter / 60) / 60;

    println!(
        "Total time (HH:MM:SS): {:0>2}:{:0>2}:{:0>2}",
        hours, minutes, seconds
    );

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();
    let path_string = ".devlog";

    match args.command {
        Commands::In => clock_in(path_string)?,
        Commands::Out => clock_out(path_string)?,
        Commands::Time => get_time(path_string)?,
    }

    Ok(())
}
