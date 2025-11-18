use sea_orm::*;
use sea_orm::sea_query::Expr;
use serde::{Deserialize, Serialize};
use entity_voltech::{test_results, prelude::*};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResultWithMeasurements {
    pub id: i64,
    pub part: String,
    pub operator: String,
    pub batch: String,
    pub date: String,
    pub serial_num: String,
    pub result_num: i64,
    pub pass_fail: String,
    pub time: Option<String>,
    pub retries: Option<String>,
    pub file_path: String,
    pub measurements: serde_json::Value,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResultSummary {
    pub id: i64,
    pub part: String,
    pub batch: String,
    pub date: String,
    pub serial_num: String,
    pub result_num: i64,
    pub pass_fail: String,
    pub time: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

/// Get tests by serial number range for a specific part
pub async fn get_tests_by_serial_range(
    db: &DatabaseConnection,
    part: &str,
    serial_start: &str,
    serial_end: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .filter(test_results::Column::Part.eq(part))
        .filter(test_results::Column::SerialNum.between(serial_start, serial_end))
        .order_by_desc(test_results::Column::CreatedAt)
        .all(db)
        .await
}

/// Get tests by exact serial number with optional part filter
pub async fn get_tests_by_serial(
    db: &DatabaseConnection,
    serial_num: &str,
    part: Option<&str>,
) -> Result<Vec<test_results::Model>, DbErr> {
    let mut query = TestResults::find().filter(test_results::Column::SerialNum.eq(serial_num));

    if let Some(p) = part {
        query = query.filter(test_results::Column::Part.eq(p));
    }

    query
        .order_by_desc(test_results::Column::CreatedAt)
        .all(db)
        .await
}

/// Get failed tests with optional filters
pub async fn get_failed_tests(
    db: &DatabaseConnection,
    part: Option<&str>,
    date_from: Option<&str>,
    date_to: Option<&str>,
    limit: Option<u64>,
) -> Result<Vec<test_results::Model>, DbErr> {
    let mut query = TestResults::find().filter(test_results::Column::PassFail.ne("Pass"));

    if let Some(p) = part {
        query = query.filter(test_results::Column::Part.eq(p));
    }

    if let Some(from) = date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    query = query.order_by_desc(test_results::Column::CreatedAt);

    if let Some(lim) = limit {
        query = query.limit(lim);
    }

    query.all(db).await
}

/// Get tests by date range
pub async fn get_tests_by_date_range(
    db: &DatabaseConnection,
    date_from: &str,
    date_to: &str,
    part: Option<&str>,
) -> Result<Vec<test_results::Model>, DbErr> {
    let mut query = TestResults::find()
        .filter(test_results::Column::Date.gte(date_from))
        .filter(test_results::Column::Date.lte(date_to));

    if let Some(p) = part {
        query = query.filter(test_results::Column::Part.eq(p));
    }

    query
        .order_by_desc(test_results::Column::Date)
        .order_by_desc(test_results::Column::CreatedAt)
        .all(db)
        .await
}

/// Get tests by file path (for reprocessing checks)
pub async fn get_tests_by_file(
    db: &DatabaseConnection,
    file_path: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .filter(test_results::Column::FilePath.eq(file_path))
        .order_by_asc(test_results::Column::ResultNum)
        .all(db)
        .await
}

/// Search tests with flexible filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestSearchFilter {
    pub part: Option<String>,
    pub batch: Option<String>,
    pub serial_num: Option<String>,
    pub serial_range: Option<(String, String)>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub pass_fail: Option<String>,
    pub operator: Option<String>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

pub async fn search_tests(
    db: &DatabaseConnection,
    filter: TestSearchFilter,
) -> Result<Vec<test_results::Model>, DbErr> {
    let mut query = TestResults::find();

    if let Some(part) = filter.part {
        query = query.filter(test_results::Column::Part.eq(part));
    }

    if let Some(batch) = filter.batch {
        query = query.filter(test_results::Column::Batch.eq(batch));
    }

    if let Some(serial) = filter.serial_num {
        query = query.filter(test_results::Column::SerialNum.eq(serial));
    }

    if let Some((start, end)) = filter.serial_range {
        query = query.filter(test_results::Column::SerialNum.between(start, end));
    }

    if let Some(from) = filter.date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = filter.date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    if let Some(pf) = filter.pass_fail {
        query = query.filter(test_results::Column::PassFail.eq(pf));
    }

    if let Some(op) = filter.operator {
        query = query.filter(test_results::Column::Operator.eq(op));
    }

    query = query.order_by_desc(test_results::Column::CreatedAt);

    if let Some(limit) = filter.limit {
        query = query.limit(limit);
    }

    if let Some(offset) = filter.offset {
        query = query.offset(offset);
    }

    query.all(db).await
}

/// Count tests matching filter (for pagination)
pub async fn count_tests(
    db: &DatabaseConnection,
    filter: TestSearchFilter,
) -> Result<u64, DbErr> {
    let mut query = TestResults::find();

    if let Some(part) = filter.part {
        query = query.filter(test_results::Column::Part.eq(part));
    }

    if let Some(batch) = filter.batch {
        query = query.filter(test_results::Column::Batch.eq(batch));
    }

    if let Some(serial) = filter.serial_num {
        query = query.filter(test_results::Column::SerialNum.eq(serial));
    }

    if let Some((start, end)) = filter.serial_range {
        query = query.filter(test_results::Column::SerialNum.between(start, end));
    }

    if let Some(from) = filter.date_from {
        query = query.filter(test_results::Column::Date.gte(from));
    }

    if let Some(to) = filter.date_to {
        query = query.filter(test_results::Column::Date.lte(to));
    }

    if let Some(pf) = filter.pass_fail {
        query = query.filter(test_results::Column::PassFail.eq(pf));
    }

    if let Some(op) = filter.operator {
        query = query.filter(test_results::Column::Operator.eq(op));
    }

    query.count(db).await
}
