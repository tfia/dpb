mod cli;
mod db;

use actix_web::{middleware::Logger, web, App, HttpServer};
use anyhow::Result;
use clap::Parser;
use env_logger;
use log;
use redb::Database;
use short_crypt::ShortCrypt;

use cli::{Cli, Config};
use db::TABLE;
use dpb::api::{add, query};

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
            .app_data(web::Data::new(sc.clone()))
            .service(add::api_scope())
            .service(query::api_scope())
    })
    .bind((bind_address, bind_port))?
    .run()
    .await?;

    Ok(())
}
