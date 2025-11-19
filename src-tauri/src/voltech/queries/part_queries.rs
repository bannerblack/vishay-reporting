use sea_orm::*;
use serde::{Deserialize, Serialize};
use entity_voltech::test_results;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct PartSummary {
    pub part: String,
    pub total_batches: i64,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
    pub first_date: String,
    pub last_date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct PartListItem {
    pub part: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
}

/// Get all parts with test statistics
pub async fn get_all_parts(
    db: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<PartListItem>, DbErr> {
    let limit = limit.unwrap_or(100);

    let sql = r#"
        SELECT 
            part,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate
        FROM test_results
        GROUP BY part
        ORDER BY total_tests DESC
        LIMIT ?
    "#;

    let results = PartListItem::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [(limit as i64).into()],
    ))
    .all(db)
    .await?;

    Ok(results)
}

/// Get detailed part summary with batch count and date range
pub async fn get_part_summary(
    db: &DatabaseConnection,
    part: &str,
) -> Result<Option<PartSummary>, DbErr> {
    let sql = r#"
        SELECT 
            part,
            COUNT(DISTINCT batch) as total_batches,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate,
            MIN(date) as first_date,
            MAX(date) as last_date
        FROM test_results
        WHERE part = ?
        GROUP BY part
    "#;

    let result = PartSummary::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [part.into()],
    ))
    .one(db)
    .await?;

    Ok(result)
}

/// Search parts by pattern
pub async fn search_parts(
    db: &DatabaseConnection,
    pattern: &str,
    limit: Option<u64>,
) -> Result<Vec<String>, DbErr> {
    let limit = limit.unwrap_or(50);

    #[derive(Debug, FromQueryResult)]
    struct PartResult {
        part: String,
    }

    let sql = r#"
        SELECT DISTINCT part
        FROM test_results
        WHERE part LIKE ?
        ORDER BY part ASC
        LIMIT ?
    "#;

    let search_pattern = format!("%{}%", pattern);
    let results = PartResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [search_pattern.into(), (limit as i64).into()],
    ))
    .all(db)
    .await?;

    Ok(results.into_iter().map(|r| r.part).collect())
}

/// Get all unique part numbers
pub async fn get_all_part_numbers(
    db: &DatabaseConnection,
) -> Result<Vec<String>, DbErr> {
    #[derive(Debug, FromQueryResult)]
    struct PartResult {
        part: String,
    }

    let sql = r#"
        SELECT DISTINCT part
        FROM test_results
        ORDER BY part ASC
    "#;

    let results = PartResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [],
    ))
    .all(db)
    .await?;

    Ok(results.into_iter().map(|r| r.part).collect())
}

/// Get part statistics by date range
pub async fn get_part_stats_by_date(
    db: &DatabaseConnection,
    part: &str,
    date_from: &str,
    date_to: &str,
) -> Result<Option<PartSummary>, DbErr> {
    let sql = r#"
        SELECT 
            part,
            COUNT(DISTINCT batch) as total_batches,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate,
            MIN(date) as first_date,
            MAX(date) as last_date
        FROM test_results
        WHERE part = ? AND date >= ? AND date <= ?
        GROUP BY part
    "#;

    let result = PartSummary::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [part.into(), date_from.into(), date_to.into()],
    ))
    .one(db)
    .await?;

    Ok(result)
}
