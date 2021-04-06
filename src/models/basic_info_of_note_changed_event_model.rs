use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct BasicInfoOfNoteChangedEventModel {
pub event_id:Uuid,
#[serde(with = "ts_milliseconds")]
pub event_timestamp: DateTime<Utc>,
pub id: String,
pub text:String,
pub title:String,

}