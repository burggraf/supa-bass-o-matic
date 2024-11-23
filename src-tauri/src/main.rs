#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use serde_json::Value;
use tokio_postgres::{NoTls, Row};
use tokio_postgres::types::Type;
use uuid::Uuid;
use rust_decimal::Decimal;
use time::OffsetDateTime;

#[derive(Debug, Serialize)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

async fn format_row_value(row: &Row, i: usize, col_type: &Type) -> String {
    match col_type {
        &Type::BOOL => {
            match row.get::<_, Option<bool>>(i) {
                Some(b) => b.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::INT2 => {
            match row.get::<_, Option<i16>>(i) {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::INT4 => {
            match row.get::<_, Option<i32>>(i) {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::INT8 => {
            match row.get::<_, Option<i64>>(i) {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::FLOAT4 | &Type::FLOAT8 => {
            match row.get::<_, Option<f64>>(i) {
                Some(f) => f.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::NUMERIC => {
            match row.get::<_, Option<Decimal>>(i) {
                Some(d) => d.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::UUID => {
            match row.get::<_, Option<Uuid>>(i) {
                Some(uuid) => uuid.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::TIMESTAMP | &Type::TIMESTAMPTZ => {
            match row.get::<_, Option<OffsetDateTime>>(i) {
                Some(ts) => ts.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::JSON | &Type::JSONB => {
            match row.get::<_, Option<Value>>(i) {
                Some(json) => json.to_string(),
                None => "NULL".to_string(),
            }
        },
        _ => {
            match row.get::<_, Option<String>>(i) {
                Some(s) => s,
                None => "NULL".to_string(),
            }
        }
    }
}

#[tauri::command]
async fn execute_query(connection_string: String, query: String) -> Result<QueryResult, String> {
    println!("Connecting to database...");
    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls)
        .await
        .map_err(|e| {
            println!("Connection error: {}", e);
            e.to_string()
        })?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    println!("Executing query: {}", query);
    let rows = match client.query(&query, &[]).await {
        Ok(r) => {
            println!("Query successful, got {} rows", r.len());
            r
        },
        Err(e) => {
            println!("Query error: {}", e);
            return Err(e.to_string());
        }
    };

    if rows.is_empty() {
        println!("No rows returned");
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
        });
    }

    println!("Processing {} rows", rows.len());
    let columns: Vec<String> = rows[0].columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    let mut formatted_rows = Vec::new();
    for row in &rows {
        let mut formatted_row = Vec::new();
        for (i, column) in row.columns().iter().enumerate() {
            let value = format_row_value(row, i, column.type_()).await;
            formatted_row.push(value);
        }
        formatted_rows.push(formatted_row);
    }

    println!("Query complete, returning {} rows", formatted_rows.len());
    Ok(QueryResult {
        columns,
        rows: formatted_rows,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_query])
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                println!("Debug mode: skipping dev tools");
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
