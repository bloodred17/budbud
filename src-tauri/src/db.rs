use serde::{Deserialize, Serialize};
use surrealdb::engine::local::{Db, RocksDb};
use surrealdb::Surreal;
use tokio::sync::Mutex;
use crate::core::transaction::{TransactionSource, TransactionType};


#[derive(Debug)]
pub struct Database(pub Mutex<SurrealDb>);

#[derive(Debug, Clone)]
pub struct SurrealDb {
    pub db: Option<Surreal<Db>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JsonResponse<T> {
    data: Option<T>,
    error: bool,
}

impl <T> JsonResponse<T> where T: Serialize {
    pub fn new(value: Option<T>) -> JsonResponse<T> {
        match value {
            Some(x) => JsonResponse {
                data: Some(x),
                error: false,
            },
            None => JsonResponse {
                data: None,
                error: true,
            }
        }
    }

    pub fn to_string(&self, error_message: Option<&str>) -> String {
        match serde_json::to_string(&self) {
            Ok(x) => x,
            Err(_) => {
                match error_message {
                    Some(message) => message.to_string(),
                    None => String::from("Error while serializing"),
                }
            }
        }
    }
}

#[tauri::command]
pub async fn establish_connection(state: tauri::State<'_, Database>) -> Result<String, String> {
    // Db connection logic:
    // If state is None connect

    let mut database = state.0.lock().await;
    dbg!(&database);
    match &database.db {
        None => {
            let _db = match Surreal::new::<RocksDb>("/Users/ankurdutta/playground/budbud/database.db").await {
                Ok(x) => x,
                Err(_) => panic!("Unable to connect to database"),
            };
            _db.use_ns("namespace").use_db("database").await.expect("TODO: panic message");

            database.db.replace(_db);
        },
        Some(x) => println!("some"),
    };

    dbg!(&database.db);

    return Ok(String::from("Establishing connection"));
}

#[tauri::command]
pub async fn create_entry(state: tauri::State<'_, Database>) -> Result<String, String> {
    // Select all people records
    let surreal_db = &state.0.lock().await;
    let db = surreal_db.db.clone().unwrap();
    dbg!(&db);

    let transaction_source = TransactionSource::new(
        &String::from("ICICI Platinum Credit Card"),
        TransactionType::Expense
    );
    let created: Vec<TransactionSource> = db
        .create("transaction_source")
        .content::<TransactionSource>(transaction_source)
        .await
        .expect("Hello");
    dbg!(&created);

    return Ok(String::from("Creating record"));
}

#[tauri::command]
pub async fn delete_entry(table: String, id: Option<String>, state: tauri::State<'_, Database>) -> Result<String, String> {
    dbg!(&id);

    let db = get_db_from_state(&state).await.unwrap();
    dbg!(&db);

    return match id {
        Some(_id) => {
            let deleted: Option<TransactionSource> = db.delete((&table, &_id))
                .await
                .expect(format!("{} is deleted", &_id).as_str());
            dbg!(&deleted);
            let json_response = JsonResponse::new(Some(deleted));
            let error_message = format!("{} is deleted but failed to serialize the output", &table);
            Ok(json_response.to_string(Some(error_message.as_str())))
        },
        None => {
            let deleted: Vec<TransactionSource> = db.delete(&table)
                .await
                .expect("Database is deleted");
            dbg!(&deleted);
            let error_message = format!("{} is deleted but failed to serialize the output", &table);
            Ok(JsonResponse::new(Some(deleted))
                .to_string(Some(error_message.as_str())))
        }
    }

    // return Ok(String::from("Deleting record"));
}

#[tauri::command]
pub async fn read(table: String, state: tauri::State<'_, Database>) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    let transaction_sources: Vec<TransactionSource> = db.select(&table)
        .await
        .expect("something");
    // dbg!(&transaction_sources);
    let json_response = JsonResponse::new(Some(transaction_sources))
        .to_string(None);
    // dbg!(&json_response);
    Ok(json_response)
}

#[tauri::command]
pub async fn update(state: tauri::State<'_, Database>) -> Result<String, String> {
    let db = get_db_from_state(&state).await.unwrap();
    dbg!(&db);

    // Perform a custom advanced query
    let sql = r#"
        SELECT *
        FROM type::table($table)
    "#;

    let groups = db.query(sql)
        .bind(("table", "transaction_source"))
        .await.expect("query should work");
    dbg!(&groups);

    return Ok(String::from("Updating"));
}

async fn get_db_from_state(state: &tauri::State<'_, Database>) -> Option<Surreal<Db>> {
    let surreal_db = &state.0.lock().await;
    surreal_db.db.clone()
}