use chrono::{DateTime, Local};
use redb::{Database, TableDefinition, Value};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

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

pub type ExpireQueue = BinaryHeap::<Reverse<(DateTime<Local>, i64)>>;

pub fn delete_expired_data(db: &Database, eq: &mut ExpireQueue) -> Result<()> {
    let write_txn = db.begin_write()?;
    let now = Local::now();
    {
        let mut table = write_txn.open_table::<i64, PasteEntry>(TABLE)?;
        let mut expired = vec![];
        while let Some(Reverse((expire_at, id))) = eq.peek() {
            if *expire_at > now {
                break;
            }
            expired.push(*id);
            eq.pop();
        }
        for id in expired {
            table.remove(id)?;
        }
    }
    write_txn.commit()?;

    Ok(())
}

pub const TABLE: TableDefinition<i64, PasteEntry> = TableDefinition::new("paste_data");
