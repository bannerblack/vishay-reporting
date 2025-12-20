use entity_manual::manual_test_results;
use sea_orm::sea_query::Expr;
use sea_orm::*;

/// Get all unique test names for a specific FG
pub async fn get_manual_test_names_for_fg(db: &DbConn, fg: &str) -> Result<Vec<String>, DbErr> {
    let test_names = manual_test_results::Entity::find()
        .select_only()
        .column(manual_test_results::Column::Test)
        .filter(manual_test_results::Column::Fg.eq(fg))
        .distinct()
        .into_tuple::<String>()
        .all(db)
        .await?;

    let mut test_names = test_names;
    test_names.sort();
    Ok(test_names)
}

/// Get manual tests by FG and batch, optionally filtered by date
pub async fn get_manual_tests_by_fg_batch(
    db: &DbConn,
    fg: &str,
    batch: &str,
    date: Option<chrono::NaiveDate>,
) -> Result<Vec<manual_test_results::Model>, DbErr> {
    let mut query = manual_test_results::Entity::find()
        .filter(manual_test_results::Column::Fg.eq(fg))
        .filter(manual_test_results::Column::Batch.eq(batch));

    if let Some(d) = date {
        query = query.filter(manual_test_results::Column::NormalizedDate.eq(d));
    }

    query.all(db).await
}

/// Get manual tests filtered by multiple criteria
pub async fn get_manual_tests_filtered(
    db: &DbConn,
    fg: Option<&str>,
    batch: Option<&str>,
    test_name: Option<&str>,
    serial_num: Option<&str>,
    date_from: Option<chrono::NaiveDate>,
    date_to: Option<chrono::NaiveDate>,
) -> Result<Vec<manual_test_results::Model>, DbErr> {
    let mut query = manual_test_results::Entity::find();

    if let Some(f) = fg {
        query = query.filter(manual_test_results::Column::Fg.eq(f));
    }

    if let Some(b) = batch {
        query = query.filter(manual_test_results::Column::Batch.eq(b));
    }

    if let Some(t) = test_name {
        query = query.filter(manual_test_results::Column::Test.eq(t));
    }

    if let Some(s) = serial_num {
        query = query.filter(manual_test_results::Column::Sn.eq(s));
    }

    if let Some(from) = date_from {
        query = query.filter(manual_test_results::Column::NormalizedDate.gte(from));
    }

    if let Some(to) = date_to {
        query = query.filter(manual_test_results::Column::NormalizedDate.lte(to));
    }

    query
        .order_by_desc(manual_test_results::Column::CreatedAt)
        .all(db)
        .await
}

/// Get summary of manual test data for an FG
pub async fn get_manual_test_summary_for_fg(
    db: &DbConn,
    fg: &str,
) -> Result<ManualTestSummary, DbErr> {
    let tests = manual_test_results::Entity::find()
        .filter(manual_test_results::Column::Fg.eq(fg))
        .all(db)
        .await?;

    let total_tests = tests.len();
    let passed = tests
        .iter()
        .filter(|t| t.passfail.to_uppercase() == "PASS")
        .count();
    let failed = total_tests - passed;

    let unique_batches: std::collections::HashSet<_> =
        tests.iter().map(|t| t.batch.clone()).collect();

    let unique_tests: std::collections::HashSet<_> = tests.iter().map(|t| t.test.clone()).collect();

    Ok(ManualTestSummary {
        fg: fg.to_string(),
        total_tests,
        passed_tests: passed,
        failed_tests: failed,
        unique_batches: unique_batches.len(),
        unique_test_types: unique_tests.len(),
    })
}

#[derive(Debug, serde::Serialize)]
pub struct ManualTestSummary {
    pub fg: String,
    pub total_tests: usize,
    pub passed_tests: usize,
    pub failed_tests: usize,
    pub unique_batches: usize,
    pub unique_test_types: usize,
}
