use crate::AppState;
use entity_voltech::test_results as voltech_test_results;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Debug, Serialize, Deserialize)]
pub struct DebugQueryResult {
    pub total_records: i32,
    pub matching_fg: i32,
    pub matching_fg_and_serials: i32,
    pub matching_pass: i32,
    pub matching_measurement: i32,
    pub sample_records: Vec<SampleRecord>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SampleRecord {
    pub part: String,
    pub serial_num: String,
    pub pass_fail: String,
    pub measurements_preview: String,
}

#[tauri::command]
pub async fn debug_voltech_query(
    fg_number: String,
    serial_range: String,
    associated_test: String,
    state: State<'_, AppState>,
) -> Result<DebugQueryResult, String> {
    let voltech_db = &state.voltech_db;

    // Parse serial range
    let parts: Vec<&str> = serial_range.split('-').collect();
    if parts.len() != 2 {
        return Err("Invalid serial range format".to_string());
    }
    let (start, end) = match (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
        (Ok(s), Ok(e)) => (s.to_string(), e.to_string()),
        _ => return Err("Invalid serial numbers".to_string()),
    };

    // Total records
    let total_records = voltech_test_results::Entity::find()
        .count(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())
        .map(|count| count as i32)?;
    // Matching FG (using starts_with since part = FG + type + revision)
    let matching_fg = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.starts_with(&fg_number))
        .count(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())
        .map(|count| count as i32)?;
    // Matching FG and serials
    let matching_fg_and_serials = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.starts_with(&fg_number))
        .filter(voltech_test_results::Column::SerialNum.between(&start, &end))
        .count(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())
        .map(|count| count as i32)?;
    // Matching pass
    let matching_pass = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.starts_with(&fg_number))
        .filter(voltech_test_results::Column::SerialNum.between(&start, &end))
        .filter(voltech_test_results::Column::PassFail.eq("Pass"))
        .count(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())
        .map(|count| count as i32)?;
    // Matching measurement
    let matching_measurement = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.starts_with(&fg_number))
        .filter(voltech_test_results::Column::SerialNum.between(&start, &end))
        .filter(voltech_test_results::Column::PassFail.eq("Pass"))
        .filter(
            voltech_test_results::Column::Measurements
                .contains(&format!("\"{}\"", associated_test)),
        )
        .count(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())
        .map(|count| count as i32)?;

    // Get sample records for inspection
    let sample_records = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.starts_with(&fg_number))
        .limit(5)
        .all(voltech_db.as_ref())
        .await
        .map_err(|e| e.to_string())?
        .into_iter()
        .map(|r| SampleRecord {
            part: r.part,
            serial_num: r.serial_num,
            pass_fail: r.pass_fail,
            measurements_preview: r.measurements.chars().take(200).collect(),
        })
        .collect();

    Ok(DebugQueryResult {
        total_records,
        matching_fg,
        matching_fg_and_serials,
        matching_pass,
        matching_measurement,
        sample_records,
    })
}
