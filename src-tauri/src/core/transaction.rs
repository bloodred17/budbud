use std::borrow::Cow;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::core::transaction_source::TransactionSource;
use crate::db::{Database, get_db_from_state, JsonResponse};
use std::string::String;


#[derive(Serialize, Deserialize, Debug)]
pub struct Transaction {
    id: Option<Thing>,
    name: Option<String>,
    transaction_source: TransactionSource,
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
        name: &Option<String>,
        transaction_for: &TransactionSource,
        amount: &f64,
    ) -> Transaction {
        Transaction {
            id: None,
            name: name.clone(),
            transaction_source: transaction_for.clone(),
            amount: amount.clone(),
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

#[derive(Debug, Deserialize, Serialize)]
struct CreateTransactionInput {
    name: Option<String>,
    transaction_source: String,
    amount: f64,
}


#[tauri::command]
pub async fn create_transaction(
    form_data: &str,
    state: tauri::State<'_, Database>
) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    dbg!(&form_data);

    let data = serde_json::from_str::<CreateTransactionInput>(form_data);
    dbg!(&data);
    if data.is_ok() {
        let sql = format!(r#"
            SELECT *
            FROM transaction_source:{}
        "#, &data.as_ref().unwrap().transaction_source);
        dbg!(&sql);
        let mut groups = db.query(sql)
            .await.expect("query should work");
        let transaction_source = &groups.take::<Vec<TransactionSource>>(0).unwrap();
        dbg!(&transaction_source);

        if !&transaction_source.is_empty() {
            let transaction = Transaction::new(
                &data.as_ref().unwrap().name,
                transaction_source.get(0).as_ref().unwrap(),
                &data.as_ref().unwrap().amount,
            );
            // dbg!(transaction);

            let created: Vec<Transaction> = db
                .create("transaction")
                .content::<Transaction>(transaction)
                .await
                .expect("Hello");
            dbg!(&created);
            // let name = &created.get(0).unwrap().name;

            return Ok(String::from(format!("Created Tx ")));
        }

    }

    return Err(String::from(format!("Failed to create Tx")));

}


#[tauri::command]
pub async fn list_transactions(state: tauri::State<'_, Database>) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    let transactions: Vec<Transaction> = db.select("transaction")
        .await
        .expect("Transactions");
    dbg!(&transactions);
    let json_response = JsonResponse::new(Some(transactions))
        .to_string(None);
    // dbg!(&json_response);
    Ok(json_response)
}
