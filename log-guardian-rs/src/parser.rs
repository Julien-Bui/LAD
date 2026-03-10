use regex::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader};
use anyhow::Result;
use crate::models::LogEntry;

pub fn parse_log_file(path: &str) -> Result<Vec<LogEntry>> 
{
    let mut entries = Vec::new();
    let pattern = Regex::new(r#"(?P<ip>\d{1,3}(?:\.\d{1,3}){3}).*?\s(?P<status>\d{3})\s(?P<size>\d+|-)"#)?;

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        if let Some(caps) = pattern.captures(&line)
        {
            entries.push(LogEntry {
                ip: caps["ip"].to_string(),
                status: caps["status"].parse().unwrap_or(0.0),
                size: caps["size"].parse().unwrap_or(0.0),
            });
        }
    }
    Ok(entries)
}