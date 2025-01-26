use actix_web::{get, web, HttpResponse, Responder, Scope};
use redb::{Database, Error, ReadableTable, TableDefinition};
use serde::{Deserialize, Serialize};

use crate::db::{PasteEntry, TABLE};

#[derive(Deserialize, Serialize, Clone)]
pub struct QueryResponse {
    pub title: String,
    pub content: String,
    pub created_at: String,
    pub expire_at: Option<String>,
}

#[get("/{key}")]
async fn query_paste(
    db: web::Data<std::sync::Arc<Database>>,
    key: web::Path<String>,
) -> impl Responder {
    let read_txn = db.begin_read().unwrap();
    let table = read_txn.open_table::<&str, PasteEntry>(TABLE).unwrap();
    let entry = table.get(key.as_str()).unwrap();
    if entry.is_none() {
        return HttpResponse::NotFound().finish();
    }
    let entry = entry.unwrap().value();
    let response = QueryResponse {
        title: entry.title.clone(),
        content: entry.content.clone(),
        created_at: entry.created_at.to_rfc3339(),
        expire_at: entry.expire_at.map(|exp| exp.to_rfc3339()),
    };
    HttpResponse::Ok().json(response)
}

pub fn api_scope() -> Scope {
    web::scope("/query").service(query_paste)
}
