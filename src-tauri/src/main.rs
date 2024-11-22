#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::Serialize;
use tokio_postgres::{NoTls, Row};
use tokio_postgres::types::Type;
use uuid::Uuid;
use time::OffsetDateTime;
use rust_decimal::Decimal;

#[derive(Debug, Serialize)]
struct QueryResult {
    columns: Vec<String>,
    rows: Vec<Vec<String>>,
}

fn convert_to_string(row: &Row, i: usize) -> String {
    let col_type = row.columns()[i].type_();
    
    match col_type {
        &Type::UUID => {
            match row.get::<_, Option<Uuid>>(i) {
                Some(uuid) => uuid.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::BOOL => {
            match row.get::<_, Option<bool>>(i) {
                Some(b) => b.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::INT2 | &Type::INT4 | &Type::INT8 => {
            match row.get::<_, Option<i64>>(i) {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::FLOAT4 | &Type::FLOAT8 => {
            match row.get::<_, Option<f64>>(i) {
                Some(n) => n.to_string(),
                None => "NULL".to_string(),
            }
        },
        &Type::NUMERIC => {
            match row.get::<_, Option<Decimal>>(i) {
                Some(n) => n.to_string(),
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
            match row.get::<_, Option<serde_json::Value>>(i) {
                Some(json) => json.to_string(),
                None => "NULL".to_string(),
            }
        },
        _ => {
            // Default to trying string conversion for other types
            match row.get::<_, Option<String>>(i) {
                Some(s) => s,
                None => "NULL".to_string(),
            }
        }
    }
}

#[tauri::command]
async fn execute_query(connection_string: String, query: String) -> Result<QueryResult, String> {
    println!("Executing query: {}", query);
    
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

    let rows = client.query(&query, &[])
        .await
        .map_err(|e| {
            println!("Query error: {}", e);
            e.to_string()
        })?;

    println!("Query executed successfully, got {} rows", rows.len());

    if rows.is_empty() {
        return Ok(QueryResult {
            columns: vec![],
            rows: vec![],
        });
    }

    let columns: Vec<String> = rows[0]
        .columns()
        .iter()
        .map(|col| col.name().to_string())
        .collect();

    let rows_data: Vec<Vec<String>> = rows
        .iter()
        .map(|row| {
            (0..columns.len())
                .map(|i| convert_to_string(row, i))
                .collect()
        })
        .collect();

    println!("Returning {} columns and {} rows", columns.len(), rows_data.len());

    Ok(QueryResult {
        columns,
        rows: rows_data,
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![execute_query])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
