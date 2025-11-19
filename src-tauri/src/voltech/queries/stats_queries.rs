use sea_orm::*;
use serde::{Deserialize, Serialize};
use entity_voltech::test_results;

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct DailyStats {
    pub date: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
    pub total_parts: i64,
    pub total_batches: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct OperatorStats {
    pub operator: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
    pub parts_tested: i64,
    pub batches_completed: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct OverallStats {
    pub total_tests: i64,
    pub total_parts: i64,
    pub total_batches: i64,
    pub total_operators: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
}

/// Get daily statistics
pub async fn get_daily_stats(
    db: &DatabaseConnection,
    date_from: Option<&str>,
    date_to: Option<&str>,
) -> Result<Vec<DailyStats>, DbErr> {
    let mut conditions = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    if let Some(from) = date_from {
        conditions.push("date >= ?");
        params.push(from.into());
    }

    if let Some(to) = date_to {
        conditions.push("date <= ?");
        params.push(to.into());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        r#"
        SELECT 
            date,
            COUNT(*) as total_tests,
            COALESCE(SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END), 0) as passed,
            COALESCE(SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END), 0) as failed,
            COALESCE(CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / NULLIF(COUNT(*), 0) AS REAL), 0.0) as pass_rate,
            COUNT(DISTINCT part) as total_parts,
            COUNT(DISTINCT batch) as total_batches
        FROM test_results
        {}
        GROUP BY date
        ORDER BY date DESC
        "#,
        where_clause
    );

    let results = DailyStats::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &sql,
        params,
    ))
    .all(db)
    .await?;

    Ok(results)
}

/// Get operator statistics
pub async fn get_operator_stats(
    db: &DatabaseConnection,
    date_from: Option<&str>,
    date_to: Option<&str>,
) -> Result<Vec<OperatorStats>, DbErr> {
    let mut conditions = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    if let Some(from) = date_from {
        conditions.push("date >= ?");
        params.push(from.into());
    }

    if let Some(to) = date_to {
        conditions.push("date <= ?");
        params.push(to.into());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let sql = format!(
        r#"
        SELECT 
            operator,
            COUNT(*) as total_tests,
            COALESCE(SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END), 0) as passed,
            COALESCE(SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END), 0) as failed,
            COALESCE(CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / NULLIF(COUNT(*), 0) AS REAL), 0.0) as pass_rate,
            COUNT(DISTINCT part) as parts_tested,
            COUNT(DISTINCT batch) as batches_completed
        FROM test_results
        {}
        GROUP BY operator
        ORDER BY total_tests DESC
        "#,
        where_clause
    );

    let results = OperatorStats::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &sql,
        params,
    ))
    .all(db)
    .await?;

    Ok(results)
}

/// Get overall statistics
pub async fn get_overall_stats(
    db: &DatabaseConnection,
) -> Result<Option<OverallStats>, DbErr> {
    let sql = r#"
        SELECT 
            COUNT(*) as total_tests,
            COUNT(DISTINCT part) as total_parts,
            COUNT(DISTINCT batch) as total_batches,
            COUNT(DISTINCT operator) as total_operators,
            COALESCE(SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END), 0) as passed,
            COALESCE(SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END), 0) as failed,
            COALESCE(CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / NULLIF(COUNT(*), 0) AS REAL), 0.0) as pass_rate
        FROM test_results
    "#;

    let result = OverallStats::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [],
    ))
    .one(db)
    .await?;

    Ok(result)
}

/// Get statistics for a specific part
pub async fn get_part_stats(
    db: &DatabaseConnection,
    part: &str,
) -> Result<Option<OverallStats>, DbErr> {
    let sql = r#"
        SELECT 
            COUNT(*) as total_tests,
            1 as total_parts,
            COUNT(DISTINCT batch) as total_batches,
            COUNT(DISTINCT operator) as total_operators,
            COALESCE(SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END), 0) as passed,
            COALESCE(SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END), 0) as failed,
            COALESCE(CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / NULLIF(COUNT(*), 0) AS REAL), 0.0) as pass_rate
        FROM test_results
        WHERE part = ?
    "#;

    let result = OverallStats::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [part.into()],
    ))
    .one(db)
    .await?;

    Ok(result)
}

/// Get date range for available data
pub async fn get_date_range(
    db: &DatabaseConnection,
) -> Result<Option<(String, String)>, DbErr> {
    #[derive(Debug, FromQueryResult)]
    struct DateRange {
        min_date: String,
        max_date: String,
    }

    let sql = r#"
        SELECT 
            MIN(date) as min_date,
            MAX(date) as max_date
        FROM test_results
    "#;

    let result = DateRange::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [],
    ))
    .one(db)
    .await?;

    Ok(result.map(|r| (r.min_date, r.max_date)))
}
