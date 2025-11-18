use sea_orm::*;
use sea_orm::sea_query::Expr;
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
    pub first_test_time: Option<String>,
    pub last_test_time: Option<String>,
    pub created_at: chrono::NaiveDateTime,
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
    pub created_at: chrono::NaiveDateTime,
}

/// Get 20 most recent batches for a part number
pub async fn get_recent_batches_for_part(
    db: &DatabaseConnection,
    part: &str,
    limit: Option<u64>,
) -> Result<Vec<BatchListItem>, DbErr> {
    let limit = limit.unwrap_or(20);

    let results = TestResults::find()
        .filter(test_results::Column::Part.eq(part))
        .column_as(test_results::Column::Batch, "batch")
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Date, "date")
        .column_as(test_results::Column::Operator, "operator")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).eq("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "passed",
        )
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "failed",
        )
        .column_as(
            Expr::cust_with_values(
                "CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL)",
                []
            ),
            "pass_rate",
        )
        .column_as(test_results::Column::CreatedAt.max(), "created_at")
        .group_by(test_results::Column::Batch)
        .group_by(test_results::Column::Part)
        .group_by(test_results::Column::Date)
        .group_by(test_results::Column::Operator)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .limit(limit)
        .into_model::<BatchListItem>()
        .all(db)
        .await?;

    Ok(results)
}

/// Get detailed batch summary with test times
pub async fn get_batch_details(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Option<BatchSummary>, DbErr> {
    let result = TestResults::find()
        .filter(test_results::Column::Batch.eq(batch))
        .column_as(test_results::Column::Batch, "batch")
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Date, "date")
        .column_as(test_results::Column::Operator, "operator")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).eq("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "passed",
        )
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "failed",
        )
        .column_as(
            Expr::cust_with_values(
                "CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL)",
                []
            ),
            "pass_rate",
        )
        .column_as(test_results::Column::Time.min(), "first_test_time")
        .column_as(test_results::Column::Time.max(), "last_test_time")
        .column_as(test_results::Column::CreatedAt.max(), "created_at")
        .group_by(test_results::Column::Batch)
        .group_by(test_results::Column::Part)
        .group_by(test_results::Column::Date)
        .group_by(test_results::Column::Operator)
        .into_model::<BatchSummary>()
        .one(db)
        .await?;

    Ok(result)
}

/// Get all tests for a batch, grouped and sorted by time
pub async fn get_batch_tests_grouped_by_time(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .filter(test_results::Column::Batch.eq(batch))
        .order_by_asc(test_results::Column::Time)
        .order_by_asc(test_results::Column::ResultNum)
        .all(db)
        .await
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchTestGroup {
    pub time: Option<String>,
    pub tests: Vec<test_results::Model>,
}

/// Get batch tests grouped by time slot
pub async fn get_batch_tests_grouped(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Vec<BatchTestGroup>, DbErr> {
    let tests = get_batch_tests_grouped_by_time(db, batch).await?;

    // Group by time
    let mut grouped: Vec<BatchTestGroup> = Vec::new();
    let mut current_time: Option<String> = None;
    let mut current_tests: Vec<test_results::Model> = Vec::new();

    for test in tests {
        if current_time.as_ref() != Some(&test.time.clone().unwrap_or_default()) {
            if !current_tests.is_empty() {
                grouped.push(BatchTestGroup {
                    time: current_time.clone(),
                    tests: current_tests,
                });
                current_tests = Vec::new();
            }
            current_time = test.time.clone();
        }
        current_tests.push(test);
    }

    // Push last group
    if !current_tests.is_empty() {
        grouped.push(BatchTestGroup {
            time: current_time,
            tests: current_tests,
        });
    }

    Ok(grouped)
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

    let results = TestResults::find()
        .filter(test_results::Column::Part.eq(part))
        .select_only()
        .column(test_results::Column::Batch)
        .group_by(test_results::Column::Batch)
        .order_by_desc(test_results::Column::CreatedAt.max())
        .into_model::<BatchResult>()
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
    pub min_tests: Option<i64>,
    pub has_failures: Option<bool>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub async fn search_batches(
    db: &DatabaseConnection,
    filter: BatchSearchFilter,
) -> Result<Vec<BatchListItem>, DbErr> {
    let mut query = TestResults::find();

    if let Some(part) = &filter.part {
        query = query.filter(test_results::Column::Part.eq(part));
    }

    if let Some(from) = &filter.date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = &filter.date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    if let Some(op) = &filter.operator {
        query = query.filter(test_results::Column::Operator.eq(op));
    }

    let mut query = query
        .column_as(test_results::Column::Batch, "batch")
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Date, "date")
        .column_as(test_results::Column::Operator, "operator")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).eq("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "passed",
        )
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "failed",
        )
        .column_as(
            Expr::cust_with_values(
                "CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL)",
                []
            ),
            "pass_rate",
        )
        .column_as(test_results::Column::CreatedAt.max(), "created_at")
        .group_by(test_results::Column::Batch)
        .group_by(test_results::Column::Part)
        .group_by(test_results::Column::Date)
        .group_by(test_results::Column::Operator);

    // Note: HAVING clause for min_tests and has_failures would need to be applied after grouping
    // SeaORM 2.0 may not have direct HAVING support, so we filter in memory

    query = query.order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max());

    if let Some(limit) = filter.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = filter.offset {
        query = query.offset(offset);
    }

    let mut results = query.into_model::<BatchListItem>().all(db).await?;

    // Apply post-group filters
    if let Some(min) = filter.min_tests {
        results.retain(|b| b.total_tests >= min);
    }

    if let Some(has_fail) = filter.has_failures {
        if has_fail {
            results.retain(|b| b.failed > 0);
        } else {
            results.retain(|b| b.failed == 0);
        }
    }

    Ok(results)
}
