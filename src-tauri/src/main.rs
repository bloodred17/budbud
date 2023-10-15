// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::ops::Deref;
use surrealdb::{Connection};
use tauri::{Manager};
use tokio::sync::{mpsc, Mutex};
use tracing::info;
use crate::db::{Database, SurrealDb};

pub mod core;
pub mod db;
pub mod model;

// static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<String>>,
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    tracing_subscriber::fmt::init();

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            two_way,
            js2rs,

            core::transaction_source::create_transaction_source,
            core::transaction_source::list_transaction_sources,
            core::transaction_source::delete_transaction_source,

            db::establish_connection,
            db::create_entry,
            db::delete_entry,
            db::read,
            db::update,
        ])
        .setup(|app| {

            app.manage(Database(Mutex::new(SurrealDb { db: None })));

            tauri::async_runtime::spawn(async move {
                async_process_model(
                    async_proc_input_rx,
                    async_proc_output_tx,
                ).await
            });

            let app_handle = app.handle();
            tauri::async_runtime::spawn(async move {
                loop {
                    if let Some(output) = async_proc_output_rx.recv().await {
                        rs2js(output, &app_handle);
                    }
                }
            });

            Ok(())
        })
        .manage(AsyncProcInputTx {
            inner: Mutex::new(async_proc_input_tx),
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}


async fn async_process_model(
    mut input_rx: mpsc::Receiver<String>,
    output_tx: mpsc::Sender<String>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    while let Some(input) = input_rx.recv().await {
        let output = input;
        output_tx.send(output).await?;
    }

    Ok(())
}

fn rs2js<R: tauri::Runtime>(message: String, manager: &impl Manager<R>) {
    info!(?message, "rs2js");
    manager
        .emit_all("rs2js", format!("rs: {}", message))
        .unwrap();
}

#[tauri::command]
async fn js2rs(
    message: String,
    state: tauri::State<'_, AsyncProcInputTx>,
    state_1: tauri::State<'_, Database>,
) -> Result<(), String> {
    info!(?message, "js2rs");

    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn two_way(
    form_data: &str,
    state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    info!(?form_data, "create_transaction_source");
    let async_proc_input_tx = state.inner.lock().await;
    let message = "".to_string();
    // message = match serde_json::from_str::<TransactionSource>(form_data) {
    //     Ok(x) => {
    //         dbg!(&x);
    //         String::from("Entry Created")
    //     },
    //     Err(_) => {
    //         String::from("Failed to create")
    //     }
    // };

    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

