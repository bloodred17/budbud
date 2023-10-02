use chrono::{DateTime, Utc};

pub enum TransactionType {
    Income,
    Expense,
}

pub struct TransactionFor {
    name: String,
    transaction_type: TransactionType,
}

pub struct Transaction {
    transaction_for: TransactionFor,
    amount: f64,
    created_on: DateTime<Utc>,
    // if is_recurring true then cron string is required
    is_recurring: bool,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    chron_expression: Option<String>,
}
