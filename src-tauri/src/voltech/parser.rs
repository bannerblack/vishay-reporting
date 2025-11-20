// Parser integration with SeaORM database
use entity_voltech::test_results;
use sea_orm::sea_query::OnConflict;
use sea_orm::{entity::*, ActiveValue::NotSet, DbConn, DbErr, Set};
use serde_json::Value;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

// Optimized parser functions (from voltech_parsing)
#[inline]
pub fn is_float(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let bytes = s.as_bytes();
    let mut has_digit = false;
    let mut has_dot = false;

    for (i, &b) in bytes.iter().enumerate() {
        match b {
            b'0'..=b'9' => has_digit = true,
            b'.' => {
                if has_dot {
                    return false;
                }
                has_dot = true;
            }
            b'-' | b'+' => {
                if i != 0 {
                    return false;
                }
            }
            b'e' | b'E' => {
                return s.parse::<f64>().is_ok();
            }
            _ => return false,
        }
    }
    has_digit
}

fn combine_header(header1: &[String], header2: &[String]) -> Vec<String> {
    let max_len = header1.len().max(header2.len());
    let mut temp_header = Vec::with_capacity(max_len);
    let mut final_header = Vec::with_capacity(max_len * 2);

    for i in 0..max_len {
        let h1 = header1.get(i).map(|s| s.as_str()).unwrap_or("");
        let h2 = header2.get(i).map(|s| s.as_str()).unwrap_or("");

        if h1.is_empty() {
            temp_header.push(h2.trim().to_string());
        } else if h2.is_empty() {
            temp_header.push(h1.trim().to_string());
        } else {
            let mut combined = String::with_capacity(h1.len() + h2.len() + 1);
            combined.push_str(h1);
            combined.push(' ');
            combined.push_str(h2);
            temp_header.push(combined.trim().to_string());
        }
    }

    let mut stored_test = String::new();
    for item in &temp_header {
        if !item.is_empty() && item.as_bytes()[0].is_ascii_digit() {
            stored_test.clone_from(item);
        }

        let stored_len = 9.min(stored_test.len());

        match item.as_str() {
            "Maximum" => {
                let mut s = String::with_capacity(stored_len + 7);
                s.push_str(&stored_test[..stored_len]);
                s.push_str("Maximum");
                final_header.push(s);
            }
            "Reading" => {
                let mut s = String::with_capacity(stored_len + 7);
                s.push_str(&stored_test[..stored_len]);
                s.push_str("Reading");
                final_header.push(s);

                let mut s = String::with_capacity(stored_len + 9);
                s.push_str(&stored_test[..stored_len]);
                s.push_str("Pass/Fail");
                final_header.push(s);
            }
            "Polarity" => {
                let mut s = String::with_capacity(stored_len + 8);
                s.push_str(&stored_test[..stored_len]);
                s.push_str("Polarity");
                final_header.push(s);

                let mut s = String::with_capacity(stored_len + 17);
                s.push_str(&stored_test[..stored_len]);
                s.push_str("Polarity Pass/Fail");
                final_header.push(s);
            }
            _ if !item.is_empty() => {
                final_header.push(item.clone());
            }
            _ => {}
        }
    }

    final_header
}

fn clean_date(file_name_line: &str) -> String {
    if file_name_line.len() >= 19 {
        let bytes = file_name_line.as_bytes();
        let mut result = String::with_capacity(8);
        result.push(bytes[15] as char);
        result.push(bytes[16] as char);
        result.push('-');
        result.push(bytes[13] as char);
        result.push(bytes[14] as char);
        result.push('-');
        result.push(bytes[17] as char);
        result.push(bytes[18] as char);
        result
    } else {
        String::new()
    }
}

#[inline]
fn parse_test_date(date_line: &str) -> &str {
    if let Some(colon_pos) = date_line.find(':') {
        date_line[colon_pos + 1..].trim()
    } else {
        ""
    }
}

fn parse_csv_line(line: &str) -> Vec<String> {
    let mut fields = Vec::with_capacity(20);
    let mut current_field = String::with_capacity(32);
    let mut in_quotes = false;

    let bytes = line.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        let b = bytes[i];

        match b {
            b'"' => {
                in_quotes = !in_quotes;
            }
            b',' if !in_quotes => {
                fields.push(std::mem::take(&mut current_field));
                current_field = String::with_capacity(32);
            }
            _ => {
                current_field.push(b as char);
            }
        }
        i += 1;
    }

    fields.push(current_field);
    fields
}

fn convert_to_active_model(
    header: &[String],
    test_line: &[String],
    part: &str,
    operator: &str,
    batch: &str,
    date: &str,
    file_path: &str,
) -> test_results::ActiveModel {
    let mut measurements = HashMap::new();
    let mut serial_num = String::from("NONE");
    let mut result_num = 0i32;
    let mut pass_fail = String::new();
    let mut time = String::new();
    let mut retries = String::new();

    for (i, header_name) in header.iter().enumerate() {
        if let Some(value) = test_line.get(i) {
            match header_name.as_str() {
                "Serial #" => {
                    serial_num = if value == "NONE" {
                        String::from("NONE")
                    } else {
                        value.clone()
                    };
                }
                "Result #" => {
                    result_num = value.parse::<i32>().unwrap_or(0);
                }
                "Pass/Fail" => {
                    pass_fail = value.clone();
                }
                "Time" => {
                    time = value.clone();
                }
                "Retries" => {
                    retries = value.clone();
                }
                _ => {
                    // Store all measurement values as JSON
                    if let Ok(int_val) = value.parse::<i64>() {
                        measurements.insert(
                            header_name.clone(),
                            Value::Number(serde_json::Number::from(int_val)),
                        );
                    } else if is_float(value) {
                        if let Ok(num) = value.parse::<f64>() {
                            if let Some(json_num) = serde_json::Number::from_f64(num) {
                                measurements.insert(header_name.clone(), Value::Number(json_num));
                            } else {
                                measurements
                                    .insert(header_name.clone(), Value::String(value.clone()));
                            }
                        } else {
                            measurements.insert(header_name.clone(), Value::String(value.clone()));
                        }
                    } else {
                        measurements.insert(header_name.clone(), Value::String(value.clone()));
                    }
                }
            }
        }
    }

    // Parse date to normalized_date (DD-MM-YY format to NaiveDate)
    let normalized_date = parse_voltech_date(date);

    test_results::ActiveModel {
        id: NotSet,
        part: Set(part.to_string()),
        operator: Set(operator.to_string()),
        batch: Set(batch.to_string()),
        date: Set(date.to_string()),
        serial_num: Set(serial_num),
        result_num: Set(result_num),
        pass_fail: Set(pass_fail),
        time: Set(Some(time)),
        retries: Set(Some(retries)),
        file_path: Set(file_path.to_string()),
        measurements: Set(serde_json::to_string(&measurements).unwrap_or_else(|_| "{}".to_string())),
        created_at: NotSet,
        normalized_date: Set(normalized_date),
    }
}

/// Parse voltech date format "DD-MM-YY" to NaiveDate
/// Example: "19-11-25" -> 2025-11-19
fn parse_voltech_date(date: &str) -> Option<chrono::NaiveDate> {
    // Format: DD-MM-YY
    let parts: Vec<&str> = date.split('-').collect();
    if parts.len() != 3 {
        return None;
    }

    let day = parts[0].parse::<u32>().ok()?;
    let month = parts[1].parse::<u32>().ok()?;
    let year = parts[2].parse::<i32>().ok()?;

    // Assume 20YY for two-digit year
    let full_year = 2000 + year;

    chrono::NaiveDate::from_ymd_opt(full_year, month, day)
}

/// Parse a file and return active models ready for insertion
pub fn parse_file_to_models(
    file_path: &str,
) -> Result<Vec<test_results::ActiveModel>, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(file_path)?;
    let all_lines: Vec<&str> = content.lines().collect();

    let mut part = String::new();
    let mut operator = String::new();
    let mut batch = String::new();
    let mut date = String::new();
    let mut header: Vec<String> = Vec::new();
    let mut test_data = Vec::with_capacity(100);

    let mut i = 0;
    while i < all_lines.len() {
        let line = all_lines[i];
        let fields = parse_csv_line(line);

        if fields.is_empty() {
            i += 1;
            continue;
        }

        let first_field = &fields[0];

        if !first_field.is_empty() && first_field.as_bytes()[0].is_ascii_digit() {
            let mut test_line = fields;

            if test_line.len() > 1 && test_line[1].is_empty() {
                test_line[1] = "NONE".to_string();
            }

            if test_line.len() > 4 {
                test_line.remove(4);
            }

            let model = convert_to_active_model(
                &header, &test_line, &part, &operator, &batch, &date, file_path,
            );
            test_data.push(model);
        } else if matches!(
            first_field.as_str(),
            "Part #" | "Operator" | "Batch #" | "Result #"
        ) {
            match first_field.as_str() {
                "Part #" => {
                    part = fields.get(1).cloned().unwrap_or_default();
                    operator.clear();
                    batch.clear();
                    header.clear();
                }
                "Operator" => {
                    operator = fields.get(1).cloned().unwrap_or_default();
                }
                "Batch #" => {
                    batch = fields.get(1).cloned().unwrap_or_default();
                }
                "Result #" => {
                    if i + 1 < all_lines.len() {
                        let next_fields = parse_csv_line(all_lines[i + 1]);
                        header = combine_header(&fields, &next_fields);
                    }
                }
                _ => {}
            }
        } else if first_field.starts_with("Fil") {
            date = clean_date(first_field);
        } else if first_field.starts_with("Test Date") {
            date = parse_test_date(first_field).to_string();
        }

        i += 1;
    }

    Ok(test_data)
}

/// Parse and insert a file into the database using SeaORM bulk insert
pub async fn parse_and_insert_file(db: &DbConn, file_path: &str) -> Result<usize, DbErr> {
    // Get file metadata
    let path = Path::new(file_path);
    let metadata = fs::metadata(path)
        .map_err(|e| DbErr::Custom(format!("Failed to read file metadata: {}", e)))?;
    let file_size = metadata.len() as i32;
    let file_modified = metadata
        .modified()
        .map_err(|e| DbErr::Custom(format!("Failed to read file modified time: {}", e)))?
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|e| DbErr::Custom(format!("Invalid file modified time: {}", e)))?
        .as_secs() as i32;

    // Check if file needs processing using operations module
    let needs_processing =
        crate::voltech::operations::needs_processing(db, file_path, file_size, file_modified)
            .await?;

    if !needs_processing {
        return Ok(0); // Already processed, no changes
    }

    // Parse the file
    let models = parse_file_to_models(file_path)
        .map_err(|e| DbErr::Custom(format!("Failed to parse file: {}", e)))?;
    let count = models.len();

    // Bulk insert with conflict handling (update on duplicate)
    if !models.is_empty() {
        test_results::Entity::insert_many(models)
            .on_conflict(
                OnConflict::columns([
                    test_results::Column::FilePath,
                    test_results::Column::ResultNum,
                ])
                .do_nothing()
                .to_owned(),
            )
            .exec(db)
            .await?;
    }

    // Mark file as processed
    crate::voltech::operations::mark_file_processed(
        db,
        file_path,
        file_size,
        file_modified,
        count as i32,
    )
    .await?;

    Ok(count)
}

/// Process multiple files efficiently with retry logic
pub async fn process_files_batch(
    db: &DbConn,
    file_paths: &[String],
    max_retries: u32,
) -> Result<(usize, usize, Vec<String>), DbErr> {
    let mut total_files = 0;
    let mut total_records = 0;
    let mut errors = Vec::new();

    for file_path in file_paths {
        let mut attempts = 0;
        let mut last_error = None;

        while attempts <= max_retries {
            match parse_and_insert_file(db, file_path).await {
                Ok(count) => {
                    if count > 0 {
                        total_files += 1;
                        total_records += count;
                        println!("Processed: {} ({} records)", file_path, count);
                    }
                    last_error = None;
                    break;
                }
                Err(e) => {
                    attempts += 1;
                    last_error = Some(e);
                    if attempts <= max_retries {
                        tokio::time::sleep(tokio::time::Duration::from_secs(5 * attempts as u64))
                            .await;
                    }
                }
            }
        }

        if let Some(err) = last_error {
            let error_msg = format!(
                "Error processing {} after {} attempts: {}",
                file_path, attempts, err
            );
            eprintln!("{}", error_msg);
            errors.push(error_msg);

            // Log to database
            if let Err(log_err) =
                crate::voltech::operations::log_parse_error(db, file_path, &err.to_string(), None)
                    .await
            {
                eprintln!("Failed to log error: {}", log_err);
            }
        }
    }

    Ok((total_files, total_records, errors))
}
