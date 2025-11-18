use sea_orm::*;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use entity_voltech::{test_results, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct GlobalStats {
    pub total_tests: i64,
    pub total_parts: i64,
    pub total_batches: i64,
    pub passed_tests: i64,
    pub failed_tests: i64,
    pub pass_rate: f64,
    pub total_operators: i64,
}

/// Get global database statistics
pub async fn get_global_stats(db: &DatabaseConnection) -> Result<GlobalStats, DbErr> {
    let result = TestResults::find()
        .select_only()
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(test_results::Column::Part.count_distinct(), "total_parts")
        .column_as(test_results::Column::Batch.count_distinct(), "total_batches")
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).eq("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "passed_tests",
        )
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "failed_tests",
        )
        .column_as(
            Expr::cust_with_values(
                "CAST(SUM(CASE WHEN pass_fail = 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL)",
                []
            ),
            "pass_rate",
        )
        .column_as(test_results::Column::Operator.count_distinct(), "total_operators")
        .into_model::<GlobalStats>()
        .one(db)
        .await?
        .ok_or(DbErr::RecordNotFound("No data found".to_string()))?;

    Ok(result)
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct DailyStats {
    pub date: String,
    pub total_tests: i64,
    pub passed: i64,
    pub failed: i64,
    pub pass_rate: f64,
    pub parts_tested: i64,
    pub batches_tested: i64,
}

/// Get daily statistics across all parts
pub async fn get_daily_stats(
    db: &DatabaseConnection,
    date_from: Option<&str>,
    date_to: Option<&str>,
    limit: Option<u64>,
) -> Result<Vec<DailyStats>, DbErr> {
    let mut query = TestResults::find();

    if let Some(from) = date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    let mut query = query
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
        .column_as(test_results::Column::Part.count_distinct(), "parts_tested")
        .column_as(test_results::Column::Batch.count_distinct(), "batches_tested")
        .group_by(test_results::Column::Date)
        .order_by_desc(test_results::Column::Date);

    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    let results = query.into_model::<DailyStats>().all(db).await?;

    Ok(results)
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

/// Get statistics by operator
pub async fn get_operator_stats(
    db: &DatabaseConnection,
    date_from: Option<&str>,
    date_to: Option<&str>,
) -> Result<Vec<OperatorStats>, DbErr> {
    let mut query = TestResults::find();

    if let Some(from) = date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    let results = query
        .select_only()
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
        .column_as(test_results::Column::Part.count_distinct(), "parts_tested")
        .column_as(test_results::Column::Batch.count_distinct(), "batches_completed")
        .group_by(test_results::Column::Operator)
        .order_by_desc(test_results::Column::Id.count())
        .into_model::<OperatorStats>()
        .all(db)
        .await?;

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct TopFailedPart {
    pub part: String,
    pub total_failures: i64,
    pub total_tests: i64,
    pub failure_rate: f64,
}

/// Get parts with most failures
pub async fn get_top_failed_parts(
    db: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<TopFailedPart>, DbErr> {
    let limit = limit.unwrap_or(10);

    let results = TestResults::find()
        .select_only()
        .column_as(test_results::Column::Part, "part")
        .column_as(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
            "total_failures",
        )
        .column_as(test_results::Column::Id.count(), "total_tests")
        .column_as(
            Expr::cust_with_values(
                "CAST(SUM(CASE WHEN pass_fail != 'Pass' THEN 1.0 ELSE 0.0 END) * 100.0 / COUNT(*) AS REAL)",
                []
            ),
            "failure_rate",
        )
        .group_by(test_results::Column::Part)
        .having(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum()
            .gt(0),
        )
        .order_by_desc(
            Expr::case(
                Expr::col(test_results::Column::PassFail).ne("Pass"),
                1,
            )
            .finally(0)
            .sum(),
        )
        .limit(limit)
        .into_model::<TopFailedPart>()
        .all(db)
        .await?;

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize, FromQueryResult)]
pub struct RecentActivity {
    pub date: String,
    pub part: String,
    pub batch: String,
    pub operator: String,
    pub tests_run: i64,
    pub created_at: chrono::NaiveDateTime,
}

/// Get most recent testing activity
pub async fn get_recent_activity(
    db: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<RecentActivity>, DbErr> {
    let limit = limit.unwrap_or(20);

    let results = TestResults::find()
        .select_only()
        .column_as(test_results::Column::Date, "date")
        .column_as(test_results::Column::Part, "part")
        .column_as(test_results::Column::Batch, "batch")
        .column_as(test_results::Column::Operator, "operator")
        .column_as(test_results::Column::Id.count(), "tests_run")
        .column_as(test_results::Column::CreatedAt.max(), "created_at")
        .group_by(test_results::Column::Batch)
        .group_by(test_results::Column::Date)
        .group_by(test_results::Column::Part)
        .group_by(test_results::Column::Operator)
        .order_by_desc(Expr::col((test_results::Entity, test_results::Column::CreatedAt)).max())
        .limit(limit)
        .into_model::<RecentActivity>()
        .all(db)
        .await?;

    Ok(results)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrendData {
    pub daily_stats: Vec<DailyStats>,
    pub pass_rate_trend: Vec<f64>,
    pub volume_trend: Vec<i64>,
}

/// Get trend data for dashboard
pub async fn get_trend_data(
    db: &DatabaseConnection,
    days: u64,
) -> Result<TrendData, DbErr> {
    let daily = get_daily_stats(db, None, None, Some(days)).await?;

    let pass_rate_trend: Vec<f64> = daily.iter().map(|d| d.pass_rate).collect();
    let volume_trend: Vec<i64> = daily.iter().map(|d| d.total_tests).collect();

    Ok(TrendData {
        daily_stats: daily,
        pass_rate_trend,
        volume_trend,
    })
}
