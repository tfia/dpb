use actix_web::{get, web, HttpResponse, Responder, Scope};
use redb::Database;
use serde::{Deserialize, Serialize};
use short_crypt::ShortCrypt;

use crate::db::{PasteEntry, TABLE};
use crate::error::{ApiResult, ApiError};

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
    sc: web::Data<std::sync::Arc<ShortCrypt>>,
    key: web::Path<String>,
) -> ApiResult<impl Responder> {
    let key = key.into_inner();
    let key = match sc.decrypt_url_component(&key) {
        Ok(key) => key,
        Err(_) => {
            return Err(ApiError::new_not_found())
        }
    };
    
    let key = String::from_utf8(key).unwrap().parse::<i64>()?;
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table::<i64, PasteEntry>(TABLE)?;
    let entry = table.get(key)?;
    
    let entry = match entry {
        Some(entry) => entry.value(),
        None => {
            return Err(ApiError::new_not_found())
        }
    };
    let response = QueryResponse {
        title: entry.title.clone(),
        content: entry.content.clone(),
        created_at: entry.created_at.to_rfc3339(),
        expire_at: entry.expire_at.map(|exp| exp.to_rfc3339()),
    };
    Ok(HttpResponse::Ok().json(response))
}

pub fn api_scope() -> Scope {
    web::scope("/query").service(query_paste)
}
