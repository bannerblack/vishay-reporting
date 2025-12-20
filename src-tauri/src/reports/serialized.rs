use entity_voltech::test_results;
use entity_manual::manual_test_results;
use sea_orm::*;
use sea_orm::sea_query::Expr;
use tauri::State;
use crate::AppState;

#[tauri::command]
pub async fn get_serialized(fg: &str, rev: &str, start_serial: i32, end_serial: i32, state: State<'_, AppState>) -> Result<Vec<test_results::Model>, String> {
    let db = &*state.voltech_db;

    let results = query_serialized(fg, rev, start_serial, end_serial, db)
        .await
        .map_err(|e| format!("Failed to query serialized test results: {}", e))?;
    Ok(results)
}

/// # Query Test Results
/// - `Full Test Name`, `Serial Number Range`, `Passed`
/// - Deduplicates serial numbers and keeps the most recent test
pub async fn query_serialized(
    fg: &str, 
    rev: &str, 
    start_serial: i32, 
    end_serial: i32, 
    db: &DbConn,
) -> Result<Vec<test_results::Model>, DbErr> {
    // Build the part pattern: FG + rev (e.g., "132520FTA")
    let part_pattern = format!("{}{}", fg, rev);
    
    // First, get all matching records
    let results = test_results::Entity::find()
        .filter(test_results::Column::Part.starts_with(&part_pattern))
        .filter(
            Condition::any()
                .add(test_results::Column::SerialNum.between(
                    start_serial.to_string(),
                    end_serial.to_string(),
                ))
                .add(Expr::cust(format!(
                    "CAST(serial_num AS INTEGER) BETWEEN {} AND {}",
                    start_serial, end_serial
                )))
        )
        .filter(test_results::Column::PassFail.eq("Pass"))
        .order_by_desc(test_results::Column::CreatedAt) // Most recent first
        .all(db)
        .await?;

    // Deduplicate by serial_num, keeping only the most recent (first occurrence)
    let mut seen_serials = std::collections::HashSet::new();
    let deduplicated: Vec<test_results::Model> = results
        .into_iter()
        .filter(|r| seen_serials.insert(r.serial_num.clone()))
        .collect();

    // Sort by serial number for consistent output
    let mut sorted : Vec<test_results::Model> = deduplicated;
    sorted.sort_by(|a, b| {
        let a_num: i32 = a.serial_num.parse().unwrap_or(0);
        let b_num: i32 = b.serial_num.parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });

    Ok(sorted)
}

#[cfg(test)]
mod tests {
    use super::*;
    use sea_orm::Database;

    #[tokio::test]
    async fn test_query_test_results_by_part_and_serials() {
        // Connect to your actual voltech database
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite")
            .await
            .expect("Failed to connect to database");

        // Test with your actual data
        let results = query_serialized(
            "132520",    // fg
            "FTA",       // rev
            11664,       // start_serial
            11667,       // end_serial
            &db,
        )
        .await
        .expect("Query failed");

        println!("Found {} results", results.len());
        
        for result in results.iter() {
            println!("Part: {}, Serial: {}, Pass/Fail: {}", 
                result.part, 
                result.serial_num, 
                result.pass_fail
            );
        }

        // Add assertions
        assert!(!results.is_empty(), "Should find at least one result");
        
        // Verify all results match the part pattern
        for result in &results {
            assert!(result.part.starts_with("132520FTA"), 
                "Part should start with 132520FTA, got: {}", result.part);
            assert_eq!(result.pass_fail, "Pass", "All results should be Pass");
        }
        
        // Verify serial range
        for result in &results {
            let serial: i32 = result.serial_num.parse().expect("Serial should be numeric");
            assert!(serial >= 11664 && serial <= 11667, 
                "Serial {} should be in range 11664-11667", serial);
        }
    }

    #[tokio::test]
    async fn test_query_mtpl_2013_sn_26_95() {
        // Connect to the voltech database used in development
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite")
            .await
            .expect("Failed to connect to database");

        let results = query_serialized(
            "MTPL-2013-0023L", // fg
            "FTA",             // rev (common for this FG)
            26,                 // start_serial
            95,                 // end_serial
            &db,
        )
        .await
        .expect("Query failed");

        println!("Found {} results for MTPL-2013-0023L 26-95", results.len());
        for result in results.iter() {
            println!("Part: {}, Serial: {}, Pass/Fail: {}", result.part, result.serial_num, result.pass_fail);
        }

        // Expect at least one result; this assertion helps catch empty queries
        assert!(!results.is_empty(), "Should find at least one result for MTPL-2013-0023L 26-95");
    }

    #[tokio::test]
    async fn test_list_parts_for_mtpl_2013() {
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite")
            .await
            .expect("Failed to connect to database");

        // Look for any records where part contains the FG token
        let rows = test_results::Entity::find()
            .filter(test_results::Column::Part.contains("MTPL-2013-0023"))
            .limit(100)
            .all(&db)
            .await
            .expect("Query failed");

        println!("Found {} rows with part containing 'MTPL-2013-0023'", rows.len());
        for r in rows.iter().take(20) {
            println!("Part: {}, Serial: {}, PassFail: {}", r.part, r.serial_num, r.pass_fail);
        }

        assert!(!rows.is_empty(), "No rows found containing 'MTPL-2013-0023' in part");
    }

    #[tokio::test]
    async fn test_inspect_measurements_sn_26() {
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite")
            .await
            .expect("Failed to connect to database");

        // Find a specific record for the part and serial '026'
        let rows = test_results::Entity::find()
            .filter(test_results::Column::Part.eq("MTPL-2013-0023LFTA"))
            .filter(test_results::Column::SerialNum.eq("026"))
            .all(&db)
            .await
            .expect("Query failed");

        println!("Found {} records for SN 026", rows.len());
        if let Some(rec) = rows.first() {
            println!("Measurements JSON for SN 026: {}", rec.measurements);
            match serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(&rec.measurements) {
                Ok(map) => {
                    println!("Keys: {:?}", map.keys().collect::<Vec<_>>());
                }
                Err(e) => println!("Failed to parse measurements JSON: {}", e),
            }
        }
        assert!(!rows.is_empty(), "Should find a record for SN 026");
    }

    #[tokio::test]
    async fn test_measurement_matching_for_008_ls() {
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/voltech.sqlite")
            .await
            .expect("Failed to connect to database");

        let results = query_serialized("MTPL-2013-0023L", "FTA", 26, 26, &db).await.expect("Query failed");
        assert!(!results.is_empty(), "Expected at least one result for SN 26");

        let first = &results[0];
        let map: serde_json::Map<String, serde_json::Value> = serde_json::from_str(&first.measurements).expect("Failed to parse measurements");
        let normalized: HashMap<String, &serde_json::Value> = map.iter().map(|(k,v)| (normalize_spaces(k), v)).collect();

        // The dataset includes an '008   LS Reading' key which normalizes to '008 LS Reading'
        assert!(normalized.contains_key("008 LS Reading"), "Normalized measurements should contain '008 LS Reading'");
    }

    #[tokio::test]
    async fn test_query_manual() {
        // Connect to your actual voltech database
        let db = Database::connect("sqlite:///C:/Users/bashleigh/Desktop/ProductionProjects/REMOTE/manual.sqlite")
            .await
            .expect("Failed to connect to database");

        // Test with your actual data
        let results = query_manual_serialized(
            "132520",    // fg
            "A",       // rev
            11664,       // start_serial
            11667,       // end_serial
            &db,
        )
        .await
        .expect("Query failed");

        println!("Found {} results", results.len());
        
        for result in results.iter() {
            println!("Part: {}, Serial: {}, Pass/Fail: {}", 
                result.fg, 
                result.sn, 
                result.passfail
            );
        }

        // Add assertions
        assert!(!results.is_empty(), "Should find at least one result");
        
        // Verify all results match the part pattern
        for result in &results {
            assert!(result.fg.starts_with("13252"), 
                "Part should start with 132520FTA, got: {}", result.fg);
            assert_eq!(result.passfail, "PASS", "All results should be Pass");
        }
        
        // Verify serial range
        for result in &results {
            let serial: i32 = result.sn.parse().expect("Serial should be numeric");
            assert!(serial >= 11664 && serial <= 11667, 
                "Serial {} should be in range 11664-11667", serial);
        }
    }
}

/// # Query Test Results
/// - `Full Test Name`, `Serial Number Range`, `Passed`
/// - Deduplicates serial numbers and keeps the most recent test
pub async fn query_manual_serialized(
    fg: &str, 
    rev: &str, 
    start_serial: i32, 
    end_serial: i32, 
    db: &DbConn,
) -> Result<Vec<manual_test_results::Model>, DbErr> {
    // Build the part pattern: FG + rev (e.g., "132520FTA")
    let part_pattern = format!("{}", fg);

    // FIXME : Sort by date, not createdAt
    
    // First, get all matching records
    let results = manual_test_results::Entity::find()
        .filter(manual_test_results::Column::Fg.starts_with(&part_pattern))
        .filter(
            Condition::any()
                .add(manual_test_results::Column::Sn.between(
                    start_serial.to_string(),
                    end_serial.to_string(),
                ))
                .add(Expr::cust(format!(
                    "CAST(sn AS INTEGER) BETWEEN {} AND {}",
                    start_serial, end_serial
                )))
        )
        .filter(manual_test_results::Column::Passfail.eq("PASS"))
        .order_by_desc(manual_test_results::Column::CreatedAt) // Most recent first
        .all(db)
        .await?;

    // Deduplicate by serial_num, keeping only the most recent (first occurrence)
    let mut seen_serials = std::collections::HashSet::new();
    let deduplicated: Vec<manual_test_results::Model> = results
        .into_iter()
        .filter(|r| seen_serials.insert(r.sn.clone()))
        .collect();

    // Sort by serial number for consistent output
    let mut sorted : Vec<manual_test_results::Model> = deduplicated;
    sorted.sort_by(|a, b| {
        let a_num: i32 = a.sn.parse().unwrap_or(0);
        let b_num: i32 = b.sn.parse().unwrap_or(0);
        a_num.cmp(&b_num)
    });

    Ok(sorted)
}