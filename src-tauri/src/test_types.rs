use crate::AppState;
use entity_manual::manual_test_results;
use entity_voltech::test_results as voltech_test_results;
use sea_orm::*;
use serde::{Deserialize, Serialize};
use tauri::State;

/// Test type definition with display name and search pattern
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestType {
    pub name: &'static str,
    pub pattern: &'static str,
    pub description: &'static str,
}

/// Predefined test types with their patterns
/// Pattern is used to search in voltech measurements JSON keys and manual test names
pub const TEST_TYPES: &[TestType] = &[
    TestType {
        name: "Inductance",
        pattern: "LS",
        description: "Inductance measurements (LS series)",
    },
    TestType {
        name: "Leakage",
        pattern: "LL",
        description: "Leakage inductance measurements (LL series)",
    },
    // TODO: Add more test type mappings as they are discovered
    // Examples:
    // - DCR (DC Resistance)
    // - HiPot (High Potential Test)
    // - Turns Ratio
    // - etc.
];

/// Response containing associated tests from both databases
#[derive(Debug, Serialize, Deserialize)]
pub struct AssociatedTests {
    pub voltech_tests: Vec<String>,
    pub manual_tests: Vec<String>,
}

/// Get all available test type options (display names)
pub async fn get_test_type_options() -> Result<Vec<String>, DbErr> {
    Ok(TEST_TYPES.iter().map(|t| t.name.to_string()).collect())
}

/// Find associated tests for a given FG and test type
///
/// # Arguments
/// * `fg` - Finished Good number
/// * `test_type` - Test type name (e.g., "Inductance", "Leakage")
/// * `voltech_db` - Voltech database connection
/// * `manual_db` - Manual database connection
///
/// # Returns
/// AssociatedTests containing unique test names from both databases
pub async fn find_associated_tests(
    fg: &str,
    test_type: &str,
    voltech_db: &DbConn,
    manual_db: &DbConn,
) -> Result<AssociatedTests, DbErr> {
    // Find the test type pattern
    let pattern = TEST_TYPES
        .iter()
        .find(|t| t.name == test_type)
        .map(|t| t.pattern);

    let pattern = match pattern {
        Some(p) => p,
        None => {
            return Ok(AssociatedTests {
                voltech_tests: vec![],
                manual_tests: vec![],
            });
        }
    };

    // Query voltech database for tests with pattern in measurements JSON keys
    // The measurements field contains JSON like: {"LS 001": {...}, "LS   002": {...}}
    // We need to extract keys that contain the pattern
    let voltech_results = voltech_test_results::Entity::find()
        .filter(voltech_test_results::Column::Part.eq(fg))
        .all(voltech_db)
        .await?;

    let mut voltech_tests = std::collections::HashSet::new();
    for result in voltech_results {
        // Parse measurements JSON and extract keys containing pattern
        if let Ok(measurements) = serde_json::from_str::<serde_json::Value>(&result.measurements) {
            if let Some(obj) = measurements.as_object() {
                for key in obj.keys() {
                    if key.contains(pattern) {
                        // Normalize whitespace in key (e.g., "LS   002" -> "LS 002")
                        let normalized = key.split_whitespace().collect::<Vec<_>>().join(" ");
                        voltech_tests.insert(normalized);
                    }
                }
            }
        }
    }

    // Query manual database for tests with pattern in test name (case-insensitive)
    let manual_results = manual_test_results::Entity::find()
        .filter(manual_test_results::Column::Fg.eq(fg))
        .all(manual_db)
        .await?;

    let mut manual_tests = std::collections::HashSet::new();
    let pattern_lower = pattern.to_lowercase();
    for result in manual_results {
        if result.test.to_lowercase().contains(&pattern_lower) {
            manual_tests.insert(result.test.clone());
        }
    }

    Ok(AssociatedTests {
        voltech_tests: voltech_tests.into_iter().collect(),
        manual_tests: manual_tests.into_iter().collect(),
    })
}

// ============================================================================
// Tauri Commands
// ============================================================================

#[tauri::command]
pub async fn get_test_types() -> Result<Vec<String>, String> {
    match get_test_type_options().await {
        Ok(types) => Ok(types),
        Err(e) => Err(format!("Failed to get test types: {}", e)),
    }
}

#[tauri::command]
pub async fn find_tests_for_type(
    fg: String,
    test_type: String,
    state: State<'_, AppState>,
) -> Result<AssociatedTests, String> {
    match find_associated_tests(&fg, &test_type, &state.voltech_db, &state.manual_db).await {
        Ok(tests) => Ok(tests),
        Err(e) => Err(format!("Failed to find associated tests: {}", e)),
    }
}
