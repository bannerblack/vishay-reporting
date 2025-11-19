use sea_orm::*;
use serde::{Deserialize, Serialize};
use entity_voltech::{test_results, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct BatchSummary {
    pub batch: String,
    pub part: String,
    pub date: String,
    pub operator: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct BatchListItem {
    pub batch: String,
    pub part: String,
    pub date: String,
    pub operator: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
}

/// Get recent batches for a part number
pub async fn get_recent_batches_for_part(
    db: &DatabaseConnection,
    part: &str,
    limit: Option<u64>,
) -> Result<Vec<BatchListItem>, DbErr> {
    let limit = limit.unwrap_or(20);

    // Use raw SQL for complex aggregations with CASE expressions
    let sql = r#"
        SELECT 
            batch,
            part,
            date,
            operator,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate
        FROM test_results
        WHERE part = ?
        GROUP BY batch, part, date, operator
        ORDER BY MAX(created_at) DESC
        LIMIT ?
    "#;

    let results = BatchListItem::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [part.into(), (limit as i64).into()],
    ))
    .all(db)
    .await?;

    Ok(results)
}

/// Get detailed batch summary
pub async fn get_batch_details(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Option<BatchSummary>, DbErr> {
    let sql = r#"
        SELECT 
            batch,
            part,
            date,
            operator,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate
        FROM test_results
        WHERE batch = ?
        GROUP BY batch, part, date, operator
    "#;

    let result = BatchSummary::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [batch.into()],
    ))
    .one(db)
    .await?;

    Ok(result)
}

/// Get all tests for a batch
pub async fn get_batch_tests(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .filter(test_results::Column::Batch.eq(batch))
        .order_by_asc(test_results::Column::ResultNum)
        .all(db)
        .await
}

/// Get all unique batch numbers for a part
pub async fn get_batches_for_part(
    db: &DatabaseConnection,
    part: &str,
) -> Result<Vec<String>, DbErr> {
    #[derive(Debug, FromQueryResult)]
    struct BatchResult {
        batch: String,
    }

    let sql = r#"
        SELECT DISTINCT batch
        FROM test_results
        WHERE part = ?
        ORDER BY batch DESC
    "#;

    let results = BatchResult::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        sql,
        [part.into()],
    ))
    .all(db)
    .await?;

    Ok(results.into_iter().map(|r| r.batch).collect())
}

/// Search batches with filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchSearchFilter {
    pub part: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub operator: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub async fn search_batches(
    db: &DatabaseConnection,
    filter: BatchSearchFilter,
) -> Result<Vec<BatchListItem>, DbErr> {
    let mut conditions = Vec::new();
    let mut params: Vec<Value> = Vec::new();

    if let Some(part) = &filter.part {
        conditions.push("part = ?");
        params.push(part.clone().into());
    }

    if let Some(from) = &filter.date_from {
        conditions.push("date >= ?");
        params.push(from.clone().into());
    }

    if let Some(to) = &filter.date_to {
        conditions.push("date <= ?");
        params.push(to.clone().into());
    }

    if let Some(op) = &filter.operator {
        conditions.push("operator = ?");
        params.push(op.clone().into());
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    let limit = filter.limit.unwrap_or(100);
    let offset = filter.offset.unwrap_or(0);

    let sql = format!(
        r#"
        SELECT 
            batch,
            part,
            date,
            operator,
            COUNT(*) as total_tests,
            SUM(CASE WHEN pass_fail = 'Pass' THEN 1 ELSE 0 END) as passed,
            SUM(CASE WHEN pass_fail != 'Pass' THEN 1 ELSE 0 END) as failed,
            CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL) as pass_rate
        FROM test_results
        {}
        GROUP BY batch, part, date, operator
        ORDER BY MAX(created_at) DESC
        LIMIT ? OFFSET ?
        "#,
        where_clause
    );

    params.push((limit as i64).into());
    params.push((offset as i64).into());

    let results = BatchListItem::find_by_statement(Statement::from_sql_and_values(
        DbBackend::Sqlite,
        &sql,
        params,
    ))
    .all(db)
    .await?;

    Ok(results)
}
