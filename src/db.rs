use chrono::{DateTime, Local};
use redb::{Database, ReadableTable, TableDefinition, Value};
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct PasteEntry {
    pub title: String,
    pub content: String,
    pub created_at: DateTime<Local>,
    pub expire_at: Option<DateTime<Local>>,
}

impl Value for PasteEntry {
    type AsBytes<'a> = &'a [u8];
    type SelfType<'a> = PasteEntry;
    fn fixed_width() -> Option<usize> {
        None
    }
    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        serde_json::from_slice(data).unwrap()
    }
    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'b,
    {
        {
            let vec = serde_json::to_vec(value).unwrap();
            Box::leak(vec.into_boxed_slice())
        }
    }
    fn type_name() -> redb::TypeName {
        redb::TypeName::new("PasteEntry")
    }
}

pub fn delete_expired_data(db: &Database) -> Result<()> {
    let write_txn = db.begin_write()?;
    let now = Local::now();
    {
        let mut table = write_txn.open_table::<i64, PasteEntry>(TABLE)?;
        let mut expired = vec![];
        for entry in table.iter()? {
            let (id, paste_entry) = entry?;
            if let Some(expire_at) = paste_entry.value().expire_at {
                if expire_at < now {
                    expired.push(id.value());
                }
            }
        }
        for id in expired {
            table.remove(id)?;
        }
    }
    write_txn.commit()?;

    Ok(())
}

pub const TABLE: TableDefinition<i64, PasteEntry> = TableDefinition::new("paste_data");
