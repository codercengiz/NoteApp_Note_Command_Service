use chrono::serde::ts_milliseconds;
use chrono::{DateTime, Utc};
use serde::{Deserialize,Serialize};
use uuid::Uuid;


#[derive(Debug, Deserialize,Serialize, Clone)]
pub struct NoteCreatedEventModel {
pub event_id:Uuid,
#[serde(with = "ts_milliseconds")]
pub event_timestamp: DateTime<Utc>,
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