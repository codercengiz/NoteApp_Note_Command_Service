use crate::models::{CommandModel, CreateNoteCommandModel, EventModel, NoteCreatedEventModel};
use crate::settings::WebServerSettings;
use chrono::Utc;
use futures::TryFutureExt;
use std::{convert::Infallible, net::SocketAddr};
use uuid::Uuid;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

use super::eventstore_service::EventstoreService;

pub(crate) struct WebServer {
    pub socket_addr: SocketAddr,
    pub eventstore_service: EventstoreService,
}

impl WebServer {
    pub(crate) fn init(settings: WebServerSettings, eventstore_service: EventstoreService) -> Self {
        WebServer {
            socket_addr: settings.socket_addr,
            eventstore_service,
        }
    }
    pub(crate) async fn start(self) {
        let routes = self
            .add_note_route()
            .with(
                warp::cors()
                    .allow_any_origin()
                    .allow_methods(vec!["POST", "GET"]),
            )
            .with(warp::log("webserver"));
        warp::serve(routes).run(self.socket_addr).await;
    }

    /// POST /add-note
    fn add_note_route(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("add-note")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::header::<String>("user-id"))
            .and(Self::with_eventstore_service(
                self.eventstore_service.clone(),
            ))
            .and_then(Self::add_note_handler)
    }
    pub async fn add_note_handler(
        create_note_model: CreateNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        Self::add_note(create_note_model, user_id, eventstore_service).await
    }
    pub async fn add_note(
        create_note_model: CreateNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        let command_model = CommandModel::CreateNoteCommandModel(create_note_model);
        let event_model = Self::command_to_event(command_model);
        //Ok(Box::new(warp::reply::json(&customer)))
        eventstore_service
            .append_to_stream(&eventstore_service.client, user_id, &event_model)
            .await;
        return Ok(Box::new(warp::reply::json(&event_model)));
    }

    fn with_eventstore_service(
        eventstore_service: EventstoreService,
    ) -> impl Filter<Extract = (EventstoreService,), Error = Infallible> + Clone {
        warp::any().map(move || eventstore_service.clone())
    }
    fn command_to_event(command_model: CommandModel) -> EventModel {
        match command_model {
            CommandModel::CreateNoteCommandModel(create_note_model) => {
                EventModel::NoteCreatedEventModel(NoteCreatedEventModel {
                    event_id: Uuid::new_v4(),
                    event_timestamp: Utc::now(),
                    id: create_note_model.id,
                    pid: create_note_model.pid,
                    user_id: create_note_model.user_id,
                    create_date: create_note_model.create_date,
                    text: create_note_model.text,
                    title: create_note_model.title,
                    image: create_note_model.image,
                    file: create_note_model.file,
                })
            }
        }
    }
}
