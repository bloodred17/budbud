use serde::{Deserialize, Serialize};
use surrealdb::sql::Thing;
use crate::db::{Database, get_db_from_state, JsonResponse};


#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Income,
    Expense,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TransactionSource {
    id: Option<Thing>,
    name: String,
    transaction_type: TransactionType,
}

impl TransactionSource {
    pub fn new(
        name: &String,
        transaction_type: TransactionType,
    ) -> TransactionSource {
        TransactionSource {
            id: None,
            name: name.to_owned(),
            transaction_type,
        }
    }
}


#[tauri::command]
pub async fn create_transaction_source(
    form_data: &str,
    state: tauri::State<'_, Database>
) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    // dbg!(&db);

    let message = match serde_json::from_str::<TransactionSource>(form_data) {
        Ok(transaction_source) => {
            dbg!(&transaction_source);
            let created: Vec<TransactionSource> = db
                .create("transaction_source")
                .content::<TransactionSource>(transaction_source)
                .await
                .expect("Hello");
            dbg!(&created);
            // let id = &created.get(0).unwrap().id.as_ref().unwrap().id;
            // dbg!(id);
            let name = &created.get(0).unwrap().name;
            // dbg!(name);
            Ok(String::from(format!("Created TxSource {}", name)))
        },
        Err(_) => {
            Err(String::from("Failed to create"))
        }
    };

    return message;
}


#[tauri::command]
pub async fn list_transaction_sources(state: tauri::State<'_, Database>) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    let transaction_sources: Vec<TransactionSource> = db.select("transaction_source")
        .await
        .expect("Transaction sources");
    // dbg!(&transaction_sources);
    let json_response = JsonResponse::new(Some(transaction_sources))
        .to_string(None);
    // dbg!(&json_response);
    Ok(json_response)
}


#[tauri::command]
pub async fn delete_transaction_source(id: String, state: tauri::State<'_, Database>) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();

    let deleted: Option<TransactionSource> = db.delete(("transaction_source", &id))
        .await
        .expect(format!("{} is deleted", &id).as_str());
    dbg!(&deleted);

    let json_response = JsonResponse::new(Some(deleted));
    let error_message = "transaction source is deleted but failed to serialize the output";
    Ok(json_response.to_string(Some(error_message)))
}

