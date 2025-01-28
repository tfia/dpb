mod cli;
mod db;
mod error;

use actix_web::{middleware::{DefaultHeaders, Logger}, web, App, HttpServer, ResponseError};
use anyhow::Result;
use clap::Parser;
use db::delete_expired_data;
use env_logger;
use log;
use redb::Database;
use short_crypt::ShortCrypt;
use tokio::time::{interval, Duration};

use cli::{Cli, Config};
use dpb::api::{add, query};
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
    let db = std::sync::Arc::new(db);

    let db_clone = db.clone();
    tokio::spawn(async move {
        let mut interval = interval(Duration::from_secs(2));
        loop {
            interval.tick().await;
            if let Err(e) = delete_expired_data(&db_clone) {
                log::error!("Failed to delete expired data: {:?}", e);
            }
        }
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .wrap(DefaultHeaders::new().add(("Access-Control-Allow-Origin", "*")))
            .app_data(web::Data::new(db.clone()))
            .app_data(web::Data::new(sc.clone()))
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
