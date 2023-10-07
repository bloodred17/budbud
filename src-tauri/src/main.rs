// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::core::transaction::{TransactionSource, TransactionType};

pub mod core;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn create_transaction_source(form_data: &str) -> String {
    let x: TransactionSource = serde_json::from_str(&form_data).unwrap();
    println!("{:?}", &x);

    let transaction_for = TransactionSource::new(
        &String::from("Credit Card"),
        TransactionType::Expense,
    );
    return match serde_json::to_string(&x) {
        Ok(x) => x,
        Err(e) => String::from("")
    };
}

// #[tauri::command]
// fn create_transaction() -> String {
// }

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_transaction_source])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
