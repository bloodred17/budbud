// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]


use std::borrow::Cow;
use std::ops::Deref;
use surrealdb::{Connection, Surreal};
use surrealdb::engine::any::Any;
use surrealdb::engine::local::{Db, RocksDb};
use tauri::{Error, Manager};
use tokio::sync::{mpsc, Mutex, MutexGuard};
use tracing::info;
use once_cell::unsync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::async_runtime::JoinHandle;
use crate::core::transaction::{TransactionSource, TransactionType};

pub mod core;

// static DB: Lazy<Surreal<Db>> = Lazy::new(Surreal::init);

struct AsyncProcInputTx {
    inner: Mutex<mpsc::Sender<String>>,
}

#[derive(Debug)]
struct Database(Mutex<SurrealDb>);

#[derive(Debug)]
struct SurrealDb {
    db: Option<Surreal<Db>>
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    tracing_subscriber::fmt::init();

    let (async_proc_input_tx, async_proc_input_rx) = mpsc::channel(1);
    let (async_proc_output_tx, mut async_proc_output_rx) = mpsc::channel(1);

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![create_transaction_source, js2rs])
        .setup(|app| {
            //
            // let db: JoinHandle<Surreal<Db>> = tauri::async_runtime::spawn(async move {
            //     let db = match Surreal::new::<RocksDb>("/Users/ankurdutta/playground/surreal-demo/mydatabase.db").await {
            //         Ok(x) => x,
            //         Err(_) => panic!("Unable to connect to database"),
            //     };
            //     db.use_ns("namespace").use_db("database").await.expect("TODO: panic message");
            //     // // Create a new person with a random ID
            //     // let created: Vec<Person> = match db
            //     //     .create("person")
            //     //     .content::<Person>(Person {
            //     //         title: "Founder & CEO".into(),
            //     //         name: Name {
            //     //             first: "Tobie".into(),
            //     //             last: "Morgan Hitchcock".into(),
            //     //         },
            //     //         marketing: true,
            //     //     })
            //     //     .await {
            //     //     Ok(x) => dbg!(x),
            //     //     Err(_) => panic!("could not create person"),
            //     // };
            //     // // Select all people records
            //     // let people: Vec<Person> = db.select("person").await.expect("something");
            //     // dbg!(people);
            //     db
            // });

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

#[derive(Debug, Serialize, Deserialize)]
struct Name {
    first: Cow<'static, str>,
    last: Cow<'static, str>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Person {
    title: Cow<'static, str>,
    name: Name,
    marketing: bool,
}

#[tauri::command]
async fn js2rs(
    message: String,
    state: tauri::State<'_, AsyncProcInputTx>,
    state_1: tauri::State<'_, Database>,
) -> Result<(), String> {
    info!(?message, "js2rs");


    // Db connection logic:
    // If state is None connect

    let mut database = state_1.0.lock().await;
    dbg!(&database);
    match &database.db {
        None => {
            let _db = match Surreal::new::<RocksDb>("/Users/ankurdutta/playground/surreal-demo/mydatabase.db").await {
                Ok(x) => x,
                Err(_) => panic!("Unable to connect to database"),
            };
            _db.use_ns("namespace").use_db("database").await.expect("TODO: panic message");
            // Select all people records
            let people: Vec<Person> = _db.select("person").await.expect("something");
            dbg!(people);

            database.db.replace(_db);
        },
        Some(x) => println!("some"),
    };

    dbg!(&database.db);

    let async_proc_input_tx = state.inner.lock().await;
    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn create_transaction_source(
    form_data: &str,
    state: tauri::State<'_, AsyncProcInputTx>,
) -> Result<(), String> {
    info!(?form_data, "create_transaction_source");
    let async_proc_input_tx = state.inner.lock().await;
    let mut message = "".to_string();
    message = match serde_json::from_str::<TransactionSource>(form_data) {
        Ok(x) => {
            dbg!(&x);
            String::from("Entry Created")
        },
        Err(_) => {
            String::from("Failed to create")
        }
    };

    async_proc_input_tx
        .send(message)
        .await
        .map_err(|e| e.to_string())
}

