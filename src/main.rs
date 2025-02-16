mod cli;
mod db;
mod error;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer, ResponseError};
use anyhow::Result;
use clap::Parser;
use db::delete_expired_data;
use env_logger;
use log;
use redb::{Database, ReadableTable};
use short_crypt::ShortCrypt;
use std::cmp::Reverse;
use tokio::time::{interval, Duration};

use cli::{Cli, Config};
use dpb::api::{add, query};
use dpb::db::{PasteEntry, TABLE, ExpireQueue};
use error::ApiError;

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = Cli::parse();
    log::info!("Cli arguments: {:?}", args);
    if args.flush_data {
        // remove db.redb if exists
        std::fs::remove_file("db.redb").ok();
    }
    let (bind_address, bind_port, magic) = match args.config {
        Some(path) => {
            let config_json = std::fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&config_json)?;
            (config.bind_address, config.bind_port, config.magic)
        }
        None => ("127.0.0.1".to_string(), 12345, "magic".to_string())
    };

    let sc = std::sync::Arc::new(ShortCrypt::new(magic));

    let db = Database::create("db.redb")?;
    // Create table
    let write_txn = db.begin_write()?;
    {
        let _ = write_txn.open_table::<i64, PasteEntry>(TABLE)?;
    }
    write_txn.commit()?;
    let db = std::sync::Arc::new(db);

    let mut expire_queue = ExpireQueue::new();
    let read_txn = db.begin_read()?;
    {
        let table = read_txn.open_table::<i64, PasteEntry>(TABLE)?;
        for entry in table.iter()? {
            let (key, paste_entry) = entry?;
            if let Some(expire_at) = paste_entry.value().expire_at {
                expire_queue.push(Reverse((expire_at, key.value())));
            }
        }
    }
    let expire_queue = std::sync::Arc::new(std::sync::Mutex::new(expire_queue));
    let expire_queue_clone = expire_queue.clone();

    let db_clone = db.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            if let Err(e) = delete_expired_data(&db_clone, &mut expire_queue_clone.lock().unwrap()) {
                log::error!("Failed to delete expired data: {:?}", e);
            }
        }
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allow_any_header()
            .max_age(3600);
        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(sc.clone()))
            .app_data(web::Data::new(expire_queue.clone()))
            .service(add::api_scope())
            .service(query::api_scope())
            .default_service(web::to(|| async {
                ApiError::new_not_found().error_response()
            }))
    })
    .bind((bind_address, bind_port))?
    .run()
    .await?;

    Ok(())
}
