use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionSource {
    name: String,
    transaction_type: TransactionType,
}

impl TransactionSource {
    pub fn new(
        name: &String,
        transaction_type: TransactionType,
    ) -> TransactionSource {
        TransactionSource {
            name: name.to_owned(),
            transaction_type,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    transaction_for: TransactionSource,
    amount: f64,
    created_on: DateTime<Utc>,
    // if is_recurring true then cron string is required
    is_recurring: bool,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
    chron_expression: Option<String>,
}

impl Transaction {
    pub fn new(
        transaction_for: &TransactionSource,
        amount: f64,
    ) -> Transaction {
        Transaction {
            transaction_for: transaction_for.clone(),
            amount,
            created_on: Utc::now(),
            is_recurring: false,
            start_date: Some(Utc::now()),
            end_date: None,
            chron_expression: None
        }
    }

    pub fn set_recurring_charge(mut self, chron_expression: String, end_date: Option<DateTime<Utc>>) -> Transaction {
        self.is_recurring = true;
        self.chron_expression = Some(chron_expression);
        if end_date != None {
            self.end_date = end_date;
        }
        self
    }


}