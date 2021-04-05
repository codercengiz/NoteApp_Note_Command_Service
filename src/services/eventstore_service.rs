use chrono::Utc;
use eventstore::{
    AppendToStreamOptions, Client, Credentials, EventData, ExpectedRevision, ReadResult,
    ReadStreamOptions,
};
use futures::TryFutureExt;
use serde::{Deserialize, Serialize};

use crate::{
    models::{EventModel, NoteCreatedEventModel},
    settings::EventstoreSettings,
};
use eventstore::Error as EventStoreError;
use serde_json::error::Error as SerdeDeserilizationError;
use thiserror::Error;
use uuid::Uuid;

pub(crate) struct EventstoreService {
    pub client: Client,
}


impl EventstoreService {
    pub async fn init(settings: EventstoreSettings) -> Self {
        let clientSettings = format!(
            "esdb://{}:{}@{}",
            settings.username, settings.password, settings.host_with_parameters
        )
        .parse()
        .unwrap();
        let client = Client::create(clientSettings).await.unwrap();
        EventstoreService { client }
    }
    pub fn get_event_type(event_model: &EventModel) -> String {
        match event_model {
            EventModel::NoteCreatedEventModel(_) => "note_created".to_string(),
        }
    }

    pub fn get_event_id(event_model: &EventModel) -> Uuid {
        match event_model {
            EventModel::NoteCreatedEventModel(event) => event.event_id,
        }
    }
    pub async fn append_to_stream(
        &self,
        client: &Client,
        user_id: &str,
        data: &EventModel,
    ) -> Result<(), EventstoreServiceError> {
        // region append-to-stream

        let event = EventData::json(Self::get_event_type(&data), &data).expect("Evendata error").id(Self::get_event_id(&data));

        

        client
            .append_to_stream(format!("{}-{}", "user", user_id), &Default::default(), event)
            .map_err(|err| EventstoreServiceError::from(&err))
            .map_ok(|_| ())
            .await
    }
}

impl Clone for EventstoreService {
    fn clone(&self) -> Self {
        EventstoreService {
            client: self.client.clone(),
        }
    }
}

#[derive(Debug, Error)]
pub(crate) enum EventstoreServiceError {
    #[error("A eventstore error occurred")]
    Eventstore,
}
impl From<&EventStoreError> for EventstoreServiceError {
    fn from(err: &EventStoreError) -> Self {
        EventstoreServiceError::Eventstore
    }
}
