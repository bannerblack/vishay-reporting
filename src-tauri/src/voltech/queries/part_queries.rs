use sea_orm::*;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use entity_voltech::{test_results, prelude::*};

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
    pub latest_batch: String,
    pub latest_operator: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct PartListItem {
    pub part: String,
    pub total_tests: i64,
    pub last_tested: chrono::NaiveDateTime,
}

/// Get all unique parts with basic info
pub async fn get_all_parts(db: &DatabaseConnection) -> Result<Vec<PartListItem>, DbErr> {
    let results = TestResults::find()
        .select_only()
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(test_results::Column::CreatedAt.max(), "last_tested")
        .group_by(test_results::Column::Part)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .into_model::<PartListItem>()
        .all(db)
        .await?;

    Ok(results)
}

/// Search parts by partial match
pub async fn search_parts(
    db: &DatabaseConnection,
    search_term: &str,
    limit: Option<u64>,
) -> Result<Vec<PartListItem>, DbErr> {
    let limit = limit.unwrap_or(50);
    let pattern = format!("%{}%", search_term);

    let results = TestResults::find()
        .filter(test_results::Column::Part.like(&pattern))
        .select_only()
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(test_results::Column::CreatedAt.max(), "last_tested")
        .group_by(test_results::Column::Part)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .limit(limit)
        .into_model::<PartListItem>()
        .all(db)
        .await?;

    Ok(results)
}

/// Get comprehensive statistics for a specific part
pub async fn get_part_stats(
    db: &DatabaseConnection,
    part: &str,
) -> Result<Option<PartSummary>, DbErr> {
    let result = TestResults::find()
        .filter(test_results::Column::Part.eq(part))
        .select_only()
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Batch.count_distinct(), "total_batches")
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
        .column_as(test_results::Column::Date.min(), "first_date")
        .column_as(test_results::Column::Date.max(), "last_date")
        .column_as(test_results::Column::Batch.max(), "latest_batch")
        .column_as(test_results::Column::Operator.max(), "latest_operator")
        .into_model::<PartSummary>()
        .one(db)
        .await?;

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct PartDateStats {
    pub date: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
}

/// Get daily statistics for a part over a date range
pub async fn get_part_daily_stats(
    db: &DatabaseConnection,
    part: &str,
    date_from: Option<&str>,
    date_to: Option<&str>,
) -> Result<Vec<PartDateStats>, DbErr> {
    let mut query = TestResults::find().filter(test_results::Column::Part.eq(part));

    if let Some(from) = date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    let results = query
        .select_only()
        .column_as(test_results::Column::Date, "date")
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
        .group_by(test_results::Column::Date)
        .order_by_desc(test_results::Column::Date)
        .into_model::<PartDateStats>()
        .all(db)
        .await?;

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct SerialNumberInfo {
    pub serial_num: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub latest_batch: String,
    pub latest_date: String,
}

/// Get all serial numbers for a part
pub async fn get_serial_numbers_for_part(
    db: &DatabaseConnection,
    part: &str,
    limit: Option<u64>,
) -> Result<Vec<SerialNumberInfo>, DbErr> {
    let limit = limit.unwrap_or(100);

    let results = TestResults::find()
        .filter(test_results::Column::Part.eq(part))
        .select_only()
        .column_as(test_results::Column::SerialNum, "serial_num")
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
        .column_as(test_results::Column::Batch.max(), "latest_batch")
        .column_as(test_results::Column::Date.max(), "latest_date")
        .group_by(test_results::Column::SerialNum)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .limit(limit)
        .into_model::<SerialNumberInfo>()
        .all(db)
        .await?;

    Ok(results)
}

/// Get parts tested in date range
pub async fn get_parts_by_date_range(
    db: &DatabaseConnection,
    date_from: &str,
    date_to: &str,
) -> Result<Vec<PartListItem>, DbErr> {
    let results = TestResults::find()
        .filter(test_results::Column::Date.gte(date_from))
        .filter(test_results::Column::Date.lte(date_to))
        .select_only()
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(test_results::Column::CreatedAt.max(), "last_tested")
        .group_by(test_results::Column::Part)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .into_model::<PartListItem>()
        .all(db)
        .await?;

    Ok(results)
}
