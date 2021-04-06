
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct ChangeParentOfNoteCommandModel {
    pub command_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    pub command_timestamp: DateTime<Utc>,
    pub command_creator_id: String,
    pub command_creator_app_type: u32,
    pub id: String,
    pub pid:String,
    
}

impl ChangeParentOfNoteCommandModel {}

