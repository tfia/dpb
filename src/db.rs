use chrono::{DateTime, Local};
use redb::{TableDefinition, Value};
use serde::{Deserialize, Serialize};

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

pub const TABLE: TableDefinition<i64, PasteEntry> = TableDefinition::new("paste_data");
