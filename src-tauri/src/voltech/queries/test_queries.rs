use entity_voltech::{prelude::*, test_results};
use sea_orm::*;
use sea_query::Expr;
use serde::{Deserialize, Serialize};

/// Search tests with filters
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TestSearchFilter {
    pub part: Option<String>,
    pub batch: Option<String>,
    pub operator: Option<String>,
    pub date_from: Option<String>,
    pub date_to: Option<String>,
    pub pass_fail: Option<String>,
    pub serial_num: Option<String>,
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

    if let Some(operator) = filter.operator {
        query = query.filter(test_results::Column::Operator.eq(operator));
    }

    if let Some(date_from) = filter.date_from {
        query = query.filter(test_results::Column::Date.gte(date_from));
    }

    if let Some(date_to) = filter.date_to {
        query = query.filter(test_results::Column::Date.lte(date_to));
    }

    if let Some(pass_fail) = filter.pass_fail {
        query = query.filter(test_results::Column::PassFail.eq(pass_fail));
    }

    if let Some(serial_num) = filter.serial_num {
        let serial_trim = serial_num.trim();
        if serial_trim.chars().all(|c| c.is_ascii_digit()) {
            // Try exact string match OR numeric match (handles leading zeros)
            let n: i64 = serial_trim.parse().unwrap_or(0);
            query = query.filter(
                Condition::any()
                    .add(test_results::Column::SerialNum.eq(serial_num.clone()))
                    .add(Expr::cust(format!("CAST(serial_num AS INTEGER) = {}", n)))
            );
        } else {
            query = query.filter(test_results::Column::SerialNum.eq(serial_num));
        }
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

/// Get tests by serial number
pub async fn get_tests_by_serial(
    db: &DatabaseConnection,
    serial_num: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    let serial_trim = serial_num.trim();
    if serial_trim.chars().all(|c| c.is_ascii_digit()) {
        let n: i64 = serial_trim.parse().unwrap_or(0);
        TestResults::find()
            .filter(
                Condition::any()
                    .add(test_results::Column::SerialNum.eq(serial_num.to_string()))
                    .add(Expr::cust(format!("CAST(serial_num AS INTEGER) = {}", n))),
            )
            .order_by_asc(test_results::Column::ResultNum)
            .all(db)
            .await
    } else {
        TestResults::find()
            .filter(test_results::Column::SerialNum.eq(serial_num))
            .order_by_asc(test_results::Column::ResultNum)
            .all(db)
            .await
    }
}

/// Get failed tests only
pub async fn get_failed_tests(
    db: &DatabaseConnection,
    limit: Option<u64>,
) -> Result<Vec<test_results::Model>, DbErr> {
    let mut query = TestResults::find()
        .filter(test_results::Column::PassFail.ne("Pass"))
        .order_by_desc(test_results::Column::CreatedAt);

    if let Some(limit) = limit {
        query = query.limit(limit);
    }

    query.all(db).await
}

/// Get test by ID
pub async fn get_test_by_id(
    db: &DatabaseConnection,
    id: i32,
) -> Result<Option<test_results::Model>, DbErr> {
    TestResults::find_by_id(id).one(db).await
}

/// Get tests for a specific batch
pub async fn get_tests_by_batch(
    db: &DatabaseConnection,
    batch: &str,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .filter(test_results::Column::Batch.eq(batch))
        .order_by_asc(test_results::Column::ResultNum)
        .all(db)
        .await
}

/// Count tests by filter
pub async fn count_tests(db: &DatabaseConnection, filter: TestSearchFilter) -> Result<u64, DbErr> {
    let mut query = TestResults::find();

    if let Some(part) = filter.part {
        query = query.filter(test_results::Column::Part.eq(part));
    }

    if let Some(batch) = filter.batch {
        query = query.filter(test_results::Column::Batch.eq(batch));
    }

    if let Some(operator) = filter.operator {
        query = query.filter(test_results::Column::Operator.eq(operator));
    }

    if let Some(date_from) = filter.date_from {
        query = query.filter(test_results::Column::Date.gte(date_from));
    }

    if let Some(date_to) = filter.date_to {
        query = query.filter(test_results::Column::Date.lte(date_to));
    }

    if let Some(pass_fail) = filter.pass_fail {
        query = query.filter(test_results::Column::PassFail.eq(pass_fail));
    }

    if let Some(serial_num) = filter.serial_num {
        let serial_trim = serial_num.trim();
        if serial_trim.chars().all(|c| c.is_ascii_digit()) {
            let n: i64 = serial_trim.parse().unwrap_or(0);
            query = query.filter(
                Condition::any()
                    .add(test_results::Column::SerialNum.eq(serial_num.clone()))
                    .add(Expr::cust(format!("CAST(serial_num AS INTEGER) = {}", n)))
            );
        } else {
            query = query.filter(test_results::Column::SerialNum.eq(serial_num));
        }
    }

    query.count(db).await
}

/// Get recent tests
pub async fn get_recent_tests(
    db: &DatabaseConnection,
    limit: u64,
) -> Result<Vec<test_results::Model>, DbErr> {
    TestResults::find()
        .order_by_desc(test_results::Column::CreatedAt)
        .limit(limit)
        .all(db)
        .await
}
