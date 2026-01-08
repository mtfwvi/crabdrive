use chrono::NaiveDateTime;
use crabdrive_common::data::DataAmount;

pub(crate) struct File {
    pub name: String,
    pub size: DataAmount,
    pub last_modified_at: NaiveDateTime,
    pub created_at: NaiveDateTime,
}
