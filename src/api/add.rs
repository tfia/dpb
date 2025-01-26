use actix_web::{post, web, HttpResponse, Responder, Scope};
use redb::{Database, Error, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::db::{PasteEntry, TABLE};

#[derive(Deserialize, Serialize, Clone)]
pub struct AddRequest {
    pub title: String,
    pub content: String,
    pub expiration: Option<u64>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct AddResponse {
    pub key: String,
}

#[post("")]
async fn add_paste(
    db: web::Data<std::sync::Arc<Database>>,
    paste: web::Json<AddRequest>,
) -> impl Responder {
    let write_txn = db.begin_write().unwrap();
    
    // generate key from time
    let key = chrono::Local::now().timestamp_nanos_opt().unwrap().to_string();
    let entry = PasteEntry {
        content: paste.content.clone(),
        created_at: chrono::Local::now(),
        expire_at: paste.expiration.map(|exp| {
            chrono::Local::now() + chrono::Duration::seconds(exp as i64)
        }),
    };

    // write table
    {
        let mut table = write_txn.open_table::<&str, PasteEntry>(TABLE).unwrap();
        table.insert(key.as_str(), entry).unwrap();
    }
    write_txn.commit().unwrap();

    HttpResponse::Ok().json(AddResponse { key })
}

pub fn api_scope() -> Scope {
    web::scope("/add").service(add_paste)
}
