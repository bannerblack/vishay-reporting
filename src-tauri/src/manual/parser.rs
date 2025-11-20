use chrono::NaiveDate;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::thread;
use std::time::Duration;

/// Represents a parsed manual test result from CSV
#[derive(Debug, Clone)]
pub struct ManualTestResult {
    pub result: i32,
    pub test: String,
    pub fg: String,
    pub rev: String,
    pub batch: String,
    pub operator: String,
    pub date: String,
    pub time: String,
    pub sn: String,
    pub passfail: String,
    pub minimum: f64,
    pub reading: f64,
    pub maximum: f64,
    pub uom: String,
    pub file_path: String,
    pub normalized_date: NaiveDate,
}

/// Parse a date string in format "11/19/2025" or "MM/DD/YYYY" to NaiveDate
pub fn parse_date(date_str: &str) -> Result<NaiveDate, String> {
    // Try MM/DD/YYYY format first
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%m/%d/%Y") {
        return Ok(date);
    }

    // Try M/D/YYYY format
    if let Ok(date) = NaiveDate::parse_from_str(date_str, "%-m/%-d/%Y") {
        return Ok(date);
    }

    // Try parsing manually
    let parts: Vec<&str> = date_str.split('/').collect();
    if parts.len() == 3 {
        if let (Ok(month), Ok(day), Ok(year)) = (
            parts[0].parse::<u32>(),
            parts[1].parse::<u32>(),
            parts[2].parse::<i32>(),
        ) {
            return NaiveDate::from_ymd_opt(year, month, day)
                .ok_or_else(|| format!("Invalid date components: {}/{}/{}", month, day, year));
        }
    }

    Err(format!("Could not parse date: {}", date_str))
}

/// Retry file open with exponential backoff for network share file locks
pub fn retry_with_backoff<F, T, E>(mut f: F) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    const MAX_RETRIES: u32 = 3;
    const INITIAL_DELAY_MS: u64 = 1000;

    for attempt in 0..MAX_RETRIES {
        match f() {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt < MAX_RETRIES - 1 {
                    let delay = INITIAL_DELAY_MS * 2u64.pow(attempt);
                    thread::sleep(Duration::from_millis(delay));
                } else {
                    return Err(e);
                }
            }
        }
    }

    unreachable!()
}

/// Parse a manual test CSV file
/// CSV format: result,test,fg,rev,batch,operator,date,time,sn,passfail,minimum,reading,maximum,uom
/// Lines starting with "#" are skipped as headers
pub fn parse_manual_csv(file_path: &str) -> Result<Vec<ManualTestResult>, String> {
    // Open file with retry for network shares
    let file = retry_with_backoff(|| {
        File::open(file_path).map_err(|e| format!("Failed to open file: {}", e))
    })?;

    let reader = BufReader::new(file);
    let mut results = Vec::new();

    for (line_num, line_result) in reader.lines().enumerate() {
        let line =
            line_result.map_err(|e| format!("Failed to read line {}: {}", line_num + 1, e))?;

        // Skip empty lines and headers (lines starting with #)
        if line.trim().is_empty() || line.trim().starts_with('#') {
            continue;
        }

        // Parse CSV line
        let fields: Vec<&str> = line.split(',').collect();

        if fields.len() < 14 {
            eprintln!(
                "Warning: Line {} has only {} fields, expected 14. Skipping.",
                line_num + 1,
                fields.len()
            );
            continue;
        }

        // Parse fields
        let result_num = fields[0]
            .trim()
            .parse::<i32>()
            .map_err(|e| format!("Invalid result number on line {}: {}", line_num + 1, e))?;

        let date_str = fields[6].trim();
        let normalized_date =
            parse_date(date_str).map_err(|e| format!("Line {}: {}", line_num + 1, e))?;

        let minimum = fields[10]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid minimum on line {}: {}", line_num + 1, e))?;

        let reading = fields[11]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid reading on line {}: {}", line_num + 1, e))?;

        let maximum = fields[12]
            .trim()
            .parse::<f64>()
            .map_err(|e| format!("Invalid maximum on line {}: {}", line_num + 1, e))?;

        results.push(ManualTestResult {
            result: result_num,
            test: fields[1].trim().to_string(),
            fg: fields[2].trim().to_string(),
            rev: fields[3].trim().to_string(),
            batch: fields[4].trim().to_string(),
            operator: fields[5].trim().to_string(),
            date: date_str.to_string(),
            time: fields[7].trim().to_string(),
            sn: fields[8].trim().to_string(),
            passfail: fields[9].trim().to_string(),
            minimum,
            reading,
            maximum,
            uom: fields[13].trim().to_string(),
            file_path: file_path.to_string(),
            normalized_date,
        });
    }

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_date() {
        assert_eq!(
            parse_date("11/19/2025").unwrap(),
            NaiveDate::from_ymd_opt(2025, 11, 19).unwrap()
        );
        assert_eq!(
            parse_date("1/5/2025").unwrap(),
            NaiveDate::from_ymd_opt(2025, 1, 5).unwrap()
        );
        assert_eq!(
            parse_date("12/31/2024").unwrap(),
            NaiveDate::from_ymd_opt(2024, 12, 31).unwrap()
        );
    }

    #[test]
    fn test_parse_date_invalid() {
        assert!(parse_date("invalid").is_err());
        assert!(parse_date("13/1/2025").is_err()); // Invalid month
        assert!(parse_date("1/32/2025").is_err()); // Invalid day
    }
}
