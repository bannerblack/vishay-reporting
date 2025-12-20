use log::debug;
// use umya_spreadsheet::*;
use rust_xlsxwriter::*;
use sea_orm::{Database, DbConn};
use serde_json::Value;
use std::collections::HashMap;

use sea_orm::sea_query::ExprTrait;
use sea_orm::{ColumnTrait, EntityTrait, ModelTrait, QueryFilter};

use crate::reports::serialized::query_manual_serialized;
use crate::{reports::serialized::query_serialized, test::TestData};

use crate::{voltech, AppState};
use entity::{fg, report, test};
use entity_manual::manual_test_results;
use entity_voltech::test_results;
use tauri::State;

// Function to normalize spaces in measurement keys
fn normalize_spaces(s: &str) -> String {
    s.split_whitespace().collect::<Vec<&str>>().join(" ")
}

// Validate that for each expected manual test suffix, the manual_map has at least one PASS (case-insensitive)
fn manual_map_all_pass(
    manual_map: &HashMap<String, Vec<manual_test_results::Model>>,
    expected: &[String],
) -> bool {
    for suffix in expected {
        match manual_map.get(suffix) {
            Some(entries) => {
                let ok = entries
                    .iter()
                    .any(|m| m.passfail.eq_ignore_ascii_case("pass"));
                if !ok {
                    println!("Missing PASS for manual suffix: '{}'", suffix);
                    return false;
                }
            }
            None => {
                println!("Missing manual suffix: '{}'", suffix);
                return false;
            }
        }
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{NaiveDate, Utc};

    #[test]
    fn test_manual_map_all_pass() {
        let mut map: HashMap<String, Vec<manual_test_results::Model>> = HashMap::new();

        let m1 = manual_test_results::Model {
            id: 1,
            result: 0,
            test: "132520-LFG-DCR1".to_string(),
            fg: "132520".to_string(),
            rev: "FTA".to_string(),
            batch: "B1".to_string(),
            operator: "op".to_string(),
            date: "2025-12-12".to_string(),
            time: "12:00".to_string(),
            sn: "100".to_string(),
            passfail: "PASS".to_string(),
            minimum: 0.0,
            reading: 1.0,
            maximum: 2.0,
            uom: "Ohm".to_string(),
            file_path: "f".to_string(),
            created_at: Utc::now().into(),
            normalized_date: NaiveDate::from_ymd_opt(2025, 12, 12).unwrap(),
        };

        map.entry("DCR1".to_string()).or_default().push(m1);

        assert!(manual_map_all_pass(&map, &vec!["DCR1".to_string()]));
        assert!(!manual_map_all_pass(&map, &vec!["DCR1".to_string(), "DCR2".to_string()]));

        let m2 = manual_test_results::Model {
            id: 2,
            result: 0,
            test: "132520-LFG-DCR2".to_string(),
            fg: "132520".to_string(),
            rev: "FTA".to_string(),
            batch: "B1".to_string(),
            operator: "op".to_string(),
            date: "2025-12-12".to_string(),
            time: "12:05".to_string(),
            sn: "100".to_string(),
            passfail: "Fail".to_string(),
            minimum: 0.0,
            reading: 1.1,
            maximum: 2.0,
            uom: "Ohm".to_string(),
            file_path: "f".to_string(),
            created_at: Utc::now().into(),
            normalized_date: NaiveDate::from_ymd_opt(2025, 12, 12).unwrap(),
        };

        map.entry("DCR2".to_string()).or_default().push(m2);
        assert!(!manual_map_all_pass(&map, &vec!["DCR1".to_string(), "DCR2".to_string()]));

        // Add a PASS for DCR2
        let m3 = manual_test_results::Model {
            id: 3,
            result: 0,
            test: "132520-LFG-DCR2".to_string(),
            fg: "132520".to_string(),
            rev: "FTA".to_string(),
            batch: "B1".to_string(),
            operator: "op".to_string(),
            date: "2025-12-12".to_string(),
            time: "12:06".to_string(),
            sn: "100".to_string(),
            passfail: "pass".to_string(),
            minimum: 0.0,
            reading: 1.2,
            maximum: 2.0,
            uom: "Ohm".to_string(),
            file_path: "f".to_string(),
            created_at: Utc::now().into(),
            normalized_date: NaiveDate::from_ymd_opt(2025, 12, 12).unwrap(),
        };

        map.get_mut("DCR2").unwrap().push(m3);
        assert!(manual_map_all_pass(&map, &vec!["DCR1".to_string(), "DCR2".to_string()]));
    }
}

#[test]
fn test_normalize_spaces() {
    assert_eq!(normalize_spaces("002   LS Reading"), "002 LS Reading");
    assert_eq!(normalize_spaces("004   TR Polarity"), "004 TR Polarity");
    assert_eq!(normalize_spaces("  multiple   spaces  "), "multiple spaces");
    assert_eq!(normalize_spaces("no change"), "no change");
}

// #[tokio::test]
// async fn test_returned_tests() -> Result<(), Box<dyn std::error::Error>> {
//     let db: DbConn = Database::connect(
//         "sqlite://C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/testing.sqlite",
//     )
//     .await?;

//     let report_id = 2;

//     let tests = test::Entity::find()
//         .filter(test::Column::ReportId.eq(report_id))
//         .all(&db)
//         .await
//         .map_err(|e| format!("Failed to fetch tests: {}", e))?;

//     println!("Tests: {}", tests.len());
//     assert_eq!(tests.len(), 1);
//     Ok(())
// }

#[tauri::command]
pub async fn create_ba_report(
    fg: String,
    rev: String,
    start_serial: i32,
    end_serial: i32,
    state: State<'_, AppState>,
    tests: Vec<TestData>,
    job_number: String,
    split: String,
    date_code: String,
) -> Result<(), String> {
    xlsx(
        state,
        &fg,
        &rev,
        start_serial,
        end_serial,
        tests,
        job_number,
        split,
        date_code,
    )
    .await
    .map_err(|e| e.to_string())
}

pub async fn xlsx(
    state: State<'_, AppState>,
    fg: &str,
    rev: &str,
    start_serial: i32,
    end_serial: i32,
    tests: Vec<TestData>,
    job_number: String,
    split: String,
    date_code: String,
) -> Result<(), Box<dyn std::error::Error>> {
    // Display input parameters
    println!("=== XLSX Report Generation Parameters ===");
    println!("FG: {}", fg);
    println!("Revision: {}", rev);
    println!("Serial Range: {} - {}", start_serial, end_serial);
    println!("Number of test types: {}", tests.len());
    println!("Test types:");
    for (i, test) in tests.iter().enumerate() {
        println!(
            "  {}. {} ({} {})",
            i + 1,
            test.test_type,
            test.source_type,
            test.primary_pins.as_deref().unwrap_or("N/A")
        );
    }
    println!("==========================================");

    // Create a new Excel file object.
    let mut workbook = Workbook::new();

    let db = &*state.voltech_db;
    let man_db = &*state.manual_db;

    // FIXME : Input validation

    let test_data: Vec<test_results::Model> =
        query_serialized(fg, rev, start_serial, end_serial, db).await?;

    let manual_data : Vec<entity_manual::manual_test_results::Model> = match query_manual_serialized(fg, rev, start_serial, end_serial, man_db).await
    {
        Ok(data) => data,
        Err(e) => {
            println!("Manual data query failed: {}", e);
            return Err(Box::new(e));
        }
    };

    // Use tests parameter to determine columns - one column per test
    // println!(
    //     "Using {} tests to determine spreadsheet columns",
    //     tests.len()
    // );

    // Decode json blob from test_data.measurements

    // Add a worksheet to the workbook.
    let worksheet = workbook.add_worksheet();

    // Main Title
    let merge_title_format = Format::new()
        .set_font_name("Aptos Display")
        .set_font_size(20)
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Medium)
        .set_bold();

    // Normal Bold + Border
    let title_format = Format::new()
        .set_font_name("Aptos Narrow")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_border(FormatBorder::Medium)
        .set_font_size(12)
        .set_bold();

    // Names
    let bold_format = Format::new()
        .set_font_name("Aptos Narrow")
        .set_align(FormatAlign::VerticalCenter)
        .set_align(FormatAlign::Center)
        .set_font_size(11)
        .set_bold()
        .set_text_wrap()
        .set_border(FormatBorder::Thin);

    // Global Formatting
    worksheet.set_row_height_pixels(0, 35)?;
    worksheet.set_row_height_pixels(1, 27)?;
    worksheet.set_row_height_pixels(2, 10)?;

    worksheet.set_landscape();
    worksheet.set_print_area(0, 0, 20, 20);

    worksheet.set_margins(0.3, 0.3, 0.3, 0.3, 0.0, 0.0);

    // -------------------- Start Writing Data

    let mut col_index: u16 = 0;
    let mut test_index: usize = 0;

    // Write Headers (based on number of tests, not voltech data)
    while test_index < tests.len() {

        // Write header for every 10 columns (accounting for pages)
        if col_index % 10 == 0 {

            // Main Title
            worksheet.merge_range(
                0,
                col_index,
                0,
                col_index + 9,
                "VISHAY HIREL TEST REPORT FG XXXXXX",
                &merge_title_format,
            )?;

            // Customer PN
            worksheet.merge_range(
                1,
                col_index,
                1,
                col_index + 2,
                "Customer PN XXXXXX-XXX",
                &title_format,
            )?;

            // Job Number
            worksheet.write_with_format(1, col_index + 3, "Job", &title_format)?;
            worksheet.merge_range(
                1,
                col_index + 4,
                1,
                col_index + 5,
                &job_number,
                &title_format,
            )?;
            worksheet.write_with_format(1, col_index + 6, "Split", &title_format)?;
            worksheet.write_with_format(1, col_index + 7, &split, &title_format)?;
            worksheet.write_with_format(1, col_index + 8, "Date Code", &title_format)?;
            worksheet.write_with_format(1, col_index + 9, &date_code, &title_format)?;
            
            // Row Headers
            worksheet.write_with_format(3, col_index, "Test", &title_format)?;
            worksheet.write_with_format(4, col_index, "Source", &title_format)?;
            worksheet.write_with_format(5, col_index, "Level", &title_format)?;
            worksheet.write_with_format(6, col_index, "Frequency", &title_format)?;
            worksheet.write_with_format(7, col_index, "Minimum", &title_format)?;
            worksheet.write_with_format(8, col_index, "Maximum", &title_format)?;
            worksheet.write_with_format(9, col_index, "UoM", &title_format)?;
            worksheet.write_with_format(10, col_index, "Pins", &title_format)?;
            worksheet.write_with_format(11, col_index, "Notes", &title_format)?;
            worksheet.write_with_format(12, col_index, "SN", &title_format)?;

            // Set column width FIX ME
            worksheet.set_column_width(col_index, 14)?;

            col_index += 1;
        } else {
            // FIX ME I don't know how much of this I still need
            worksheet.set_column_width(col_index, 12)?;

            test_index += 1;
            col_index += 1;
        }
    }

    // Write test headers
    let mut test_col_index: u16 = 1;
    for test in &tests {
        worksheet.write_with_format(3, test_col_index, &test.test_type, &bold_format)?;
        worksheet.write_with_format(4, test_col_index, &test.source_type, &bold_format)?;
        worksheet.write_with_format(5, test_col_index, test.voltage, &bold_format)?;
        worksheet.write_with_format(6, test_col_index, test.frequency, &bold_format)?;
        worksheet.write_with_format(7, test_col_index, test.minimum, &bold_format)?;
        worksheet.write_with_format(8, test_col_index, test.maximum, &bold_format)?;
        worksheet.write_with_format(9, test_col_index, &test.uo_m, &bold_format)?;
        worksheet.write_with_format(10, test_col_index, test.primary_pins.clone(), &bold_format)?;
        worksheet.write_with_format(11, test_col_index, test.description.clone(), &bold_format)?;
        test_col_index += 1;
    }

    // -------------- TEST RESULTS -----------------------------------------------------------------------------//

    // Process actual test data and write to spreadsheet
    let mut row_index: u32 = 13;

    struct Part {
        sn: i32,
        voltech_data: Vec<test_results::Model>,
        manual_data: Vec<manual_test_results::Model>,
        manual_map: HashMap<String, Vec<manual_test_results::Model>>,
        valid: bool,
    }

    impl Part {
        // Validate Voltech Data
        fn is_voltech_valid(&self) -> bool {
            if !self.voltech_data.is_empty() {
                for test in &self.voltech_data {
                    if !test.pass_fail.eq_ignore_ascii_case("pass") {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }

        // Validate Manual Data
        fn is_manual_valid(&self) -> bool {
            if !self.manual_data.is_empty() {
                for test in &self.manual_data {
                    if !test.passfail.eq_ignore_ascii_case("pass") {
                        return false;
                    }
                }
                true
            } else {
                false
            }
        }

        fn test_check(&mut self) {
            self.valid = self.is_voltech_valid() && self.is_manual_valid();
        }
    };

    // Data container
    let mut parts: Vec<Part> = Vec::new();

    // Create Part per sn
    for sn in start_serial..=end_serial {
        parts.push(Part {
            sn,
            voltech_data: Vec::new(),
            manual_data: Vec::new(),
            manual_map: HashMap::new(),
            valid: false,
        });
    }

    // Convert voltech data into a hashmap, where key = sn
    let mut voltech_by_part: HashMap<i32, Vec<test_results::Model>> = HashMap::new();
    for test in test_data {
        if let Ok(sn) = test.serial_num.parse::<i32>() {
            voltech_by_part.entry(sn).or_default().push(test);
        } else {
            println!("Warning: Could not parse voltech test serial_num '{}' as i32", test.serial_num);
        }
    }

    // Build expected manual suffix list from tests (normalized)
    let expected_manual_suffixes: Vec<String> = tests
        .iter()
        .filter(|t| t.source_type == "manual")
        .map(|t| {
            let candidate = t
                .associated_test
                .as_ref()
                .map(|a| a.clone())
                .unwrap_or_else(|| t.test_type.clone());
            normalize_spaces(&candidate)
        })
        .collect();

    // Convert manual data into a hashmap, where key = sn -> (suffix -> Vec<manual_test_results::Model>)
    let mut manual_by_part: HashMap<i32, HashMap<String, Vec<manual_test_results::Model>>> = HashMap::new();
    for test in manual_data {
        // Get end of test name: eg 132520-LFG-DCR1 = DCR1
        let suffix = test.test.split('-').last().unwrap_or("").to_string();

        if let Ok(sn) = test.sn.parse::<i32>() {
            manual_by_part
                .entry(sn)
                .or_default()
                .entry(suffix)
                .or_default()
                .push(test);
        } else {
            println!("Warning: Could not parse manual test SN '{}' as i32", test.sn);
        }
    }

    // Match voltech and manual data to Parts
    for part in &mut parts {
        if let Some(e_tests) = voltech_by_part.remove(&part.sn) {
            part.voltech_data = e_tests;
        }

        // Account for multiple manual tests (grouped by suffix)
        if let Some(m_map) = manual_by_part.remove(&part.sn) {
            part.manual_map = m_map;
            // also keep a flattened list if needed elsewhere
            part.manual_data = part
                .manual_map
                .values()
                .flat_map(|v| v.clone())
                .collect();
        }
    }

    // Validate Tests using grouped manual slots
    for part in &mut parts {
        let vol_ok = part.is_voltech_valid();
        let manual_ok = manual_map_all_pass(&part.manual_map, &expected_manual_suffixes);
        part.valid = vol_ok && manual_ok;
        println!(
            "SNS : {} vol_ok:{} manual_ok:{} overall:{}",
            part.sn, vol_ok, manual_ok, part.valid
        );
    }

    // Print parts: write serial and test measurement values per row
    for (index, part) in parts.iter().enumerate() {
        let row = 13 + index as u32;
        // Write serial number in column 0
        worksheet.write_with_format(row, 0, part.sn, &bold_format)?;

        // If we have voltech data for this part, parse the first record's measurements
        if let Some(record) = part.voltech_data.first() {
            let measurements_map: serde_json::Map<String, Value> = match serde_json::from_str(&record.measurements) {
                Ok(m) => m,
                Err(e) => {
                    println!("Failed to parse measurements for SN {}: {}", part.sn, e);
                    // Fill with blanks and continue
                    let mut col_index: u16 = 1;
                    for _ in &tests {
                        worksheet.write_with_format(row, col_index, "", &bold_format)?;
                        col_index += 1;
                    }
                    continue;
                }
            };

            // Normalize keys for lookup
            let normalized_measurements: HashMap<String, &Value> = measurements_map
                .iter()
                .map(|(k, v)| (normalize_spaces(k), v))
                .collect();

            // Helper to find a measurement value given candidate keys
            let find_measurement = |candidates: &Vec<String>| -> Option<&Value> {
                // Exact match attempts
                for c in candidates {
                    if let Some(v) = normalized_measurements.get(c) {
                        return Some(*v);
                    }
                }
                // Substring match attempts (more permissive)
                for (k, v) in &normalized_measurements {
                    for c in candidates {
                        if k.contains(c) || c.contains(k) {
                            return Some(*v);
                        }
                    }
                }
                None
            };

            // Write each test's value into its column (columns start at 1)
            let mut col_index: u16 = 1;
            for test in &tests {
                // Build candidate keys: associated_test first, fallback to test_type
                let mut candidates: Vec<String> = Vec::new();
                if let Some(associated_test) = &test.associated_test {
                    candidates.push(normalize_spaces(associated_test));
                }
                candidates.push(normalize_spaces(&test.test_type));

                if let Some(value) = find_measurement(&candidates) {
                    match value {
                        Value::Number(num) => {
                            if let Some(float_val) = num.as_f64() {
                                worksheet.write_with_format(row, col_index, float_val, &bold_format)?;
                            } else {
                                worksheet.write_with_format(row, col_index, value.to_string(), &bold_format)?;
                            }
                        }
                        Value::String(s) => {
                            worksheet.write_with_format(row, col_index, s, &bold_format)?;
                        }
                        _ => {
                            worksheet.write_with_format(row, col_index, value.to_string(), &bold_format)?;
                        }
                    }
                } else {
                    // No measurement for this test - leave cell empty and log for debugging
                    println!("No measurement for SN {} candidates: {:?}", part.sn, candidates);
                    worksheet.write_with_format(row, col_index, "", &bold_format)?;
                }

                col_index += 1;
            }
        } else {
            // No voltech data - fill test columns with empty cells
            let mut col_index: u16 = 1;
            for _ in &tests {
                worksheet.write_with_format(13 + index as u32, col_index, "", &bold_format)?;
                col_index += 1;
            }
        }
    }


    // Save the file to disk.
    let path = "C:\\Users\\bashleigh\\Desktop\\ProductionProjects\\excel_writing\\custom_form.xlsx";
    workbook.save(path)?;

    // Try to open the file with the default application (Windows, macOS, Linux); ignore errors.
    let _ = std::process::Command::new("cmd")
        .args(&["/C", "start", "", path])
        .spawn();

    Ok(())
}
