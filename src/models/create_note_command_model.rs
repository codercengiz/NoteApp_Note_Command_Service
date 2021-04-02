
use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize};
use uuid::Uuid;


#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub(crate) struct CreateNoteCommandModel {
    pub command_id: Uuid,
    #[serde(with = "ts_milliseconds")]
    pub command_timestamp: DateTime<Utc>,
    pub command_creator_id: String,
    pub command_creator_app_type: u32,
    pub id: String,
    pub pid:String,
    pub user_id:String,
    #[serde(with = "ts_milliseconds")]
    pub create_date:DateTime<Utc>,
    pub text:String,
    pub title:String,
    pub image:String,
    pub file:String,
}

impl CreateNoteCommandModel {}

