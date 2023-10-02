// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::core::transaction::{TransactionFor, TransactionType};

pub mod core;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[tauri::command]
fn create_transaction_source(form_data: &str) {
    let transaction_for = TransactionFor::new(
        &String::from("Credit Card"),
        TransactionType::Expense,
    );
    let x = match serde_json::to_string(&transaction_for) {
        Ok(x) => x,
        Err(e) => String::from("")
    };
    println!("{}", x);
    println!("{}", form_data);
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
