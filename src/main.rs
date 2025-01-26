mod cli;
mod db;

use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use anyhow::Result;
use clap::Parser;
use env_logger;
use log;
use redb::{Database, Error, ReadableTable, TableDefinition};

use cli::{Cli, Config};
use db::{PasteEntry, TABLE};
use dpb::api::{add, query};

#[actix_web::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let args = Cli::parse();
    log::info!("Cli arguments: {:?}", args);
    let (bind_address, bind_port) = match args.config {
        Some(path) => {
            let config_json = std::fs::read_to_string(&path)?;
            let config: Config = serde_json::from_str(&config_json)?;
            (config.bind_address, config.bind_port)
        }
        None => ("127.0.0.1".to_string(), 12345)
    };

    let db = Database::create("db.redb")?;
    {
        let write_txn = db.begin_write()?;
        write_txn.open_table(TABLE)?;
        write_txn.commit()?;
    }
    let db = std::sync::Arc::new(db);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.clone()))
            .service(add::api_scope())
            .service(query::api_scope())
    })
    .bind((bind_address, bind_port))?
    .run()
    .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_db() -> Result<()> {
        let db = Database::create("db.redb")?;
        let time = chrono::Local::now();
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TABLE)?;
            table.insert(
                "my_key",
                PasteEntry {
                    content: "test".to_string(),
                    created_at: time,
                    expire_at: Some(time),
                },
            )?;
        }
        write_txn.commit()?;

        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TABLE)?;
        assert_eq!(
            table.get("my_key")?.unwrap().value(),
            PasteEntry {
                content: "test".to_string(),
                created_at: time,
                expire_at: Some(time)
            }
        );

        Ok(())
    }
}
