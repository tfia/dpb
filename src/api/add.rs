use actix_web::{post, web, HttpResponse, Responder, Scope};
use redb::Database;
use serde::{Deserialize, Serialize};
use short_crypt::ShortCrypt;

use crate::db::{PasteEntry, TABLE};
use crate::error::{ApiResult, ApiError, ApiErrorType};

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
    sc: web::Data<std::sync::Arc<ShortCrypt>>,
    paste: web::Json<AddRequest>,
) -> ApiResult<impl Responder> {
    let write_txn = db.begin_write()?;
    
    // generate key from time
    let key = chrono::Local::now().timestamp_nanos_opt().unwrap();
    if let Some(value) = paste.expiration {
        if value > 604800 {
            return Err(ApiError::new(
                ApiErrorType::InvalidRequest,
                "Expiration too long".to_string(),
            ))
        }
    }
    let entry = PasteEntry {
        title: paste.title.clone(),
        content: paste.content.clone(),
        created_at: chrono::Local::now(),
        expire_at: paste.expiration.map(|exp| {
            chrono::Local::now() + chrono::Duration::seconds(exp as i64)
        }),
    };

    // write table
    {
        let mut table = write_txn.open_table::<i64, PasteEntry>(TABLE)?;
        table.insert(key, entry)?;
    }
    write_txn.commit()?;

    let response_key = sc.encrypt_to_url_component(key.to_string().as_bytes());

    Ok(HttpResponse::Ok().json(AddResponse { key: response_key }))
}

pub fn api_scope() -> Scope {
    web::scope("/add").service(add_paste)
}
