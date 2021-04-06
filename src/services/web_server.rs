use crate::models::{
    BasicInfoOfNoteChangedEventModel, ChangeBasicInfoNoteCommandModel,
    ChangeParentOfNoteCommandModel, CommandModel, CreateNoteCommandModel, EventModel,
    NoteCreatedEventModel, ParentOfNoteChangedEventModel,
};
use crate::settings::WebServerSettings;
use chrono::Utc;
use std::{convert::Infallible, net::SocketAddr};
use uuid::Uuid;

use warp::{Filter, Rejection, Reply};

use super::{eventstore_service::EventstoreService, kafka_service::KafkaService};

pub(crate) struct WebServer {
    pub socket_addr: SocketAddr,
    pub eventstore_service: EventstoreService,
    pub kafka_service: KafkaService,
}

impl WebServer {
    pub(crate) fn init(
        settings: WebServerSettings,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Self {
        WebServer {
            socket_addr: settings.socket_addr,
            eventstore_service,
            kafka_service,
        }
    }
    pub(crate) async fn start(self) {
        let routes = self
            .add_note_route()
            .or(self.change_parent_of_note_route())
            .or(self.change_basic_info_note_route())
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
            .and(Self::with_kafka_service(self.kafka_service.clone()))
            .and_then(Self::add_note_handler)
    }
    fn change_parent_of_note_route(
        &self,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("change-parent-note")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::header::<String>("user-id"))
            .and(Self::with_eventstore_service(
                self.eventstore_service.clone(),
            ))
            .and(Self::with_kafka_service(self.kafka_service.clone()))
            .and_then(Self::change_parent_of_note_handler)
    }
    fn change_basic_info_note_route(
        &self,
    ) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        warp::path!("change-basic-info-note")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::header::<String>("user-id"))
            .and(Self::with_eventstore_service(
                self.eventstore_service.clone(),
            ))
            .and(Self::with_kafka_service(self.kafka_service.clone()))
            .and_then(Self::change_basic_info_note_handler)
    }

    //---------------------- end route
    pub async fn add_note_handler(
        create_note_model: CreateNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        Self::add_note(
            create_note_model,
            user_id,
            eventstore_service,
            kafka_service,
        )
        .await
    }
    pub async fn add_note(
        create_note_model: CreateNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        let command_model = CommandModel::CreateNoteCommandModel(create_note_model);
        let event_model = Self::command_to_event(command_model);
        //Ok(Box::new(warp::reply::json(&customer)))
        let _res_eventstore = eventstore_service
            .append_to_stream(&eventstore_service.client, &user_id, &event_model)
            .await;
        let json = serde_json::to_string(&event_model).expect("Serde Error!");
        let _res = kafka_service.produce_message(&json, user_id);
        return Ok(Box::new(warp::reply::json(&event_model)));
    }

    pub async fn change_parent_of_note_handler(
        change_parent_of_note_model: ChangeParentOfNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        Self::change_parent_of_note(
            change_parent_of_note_model,
            user_id,
            eventstore_service,
            kafka_service,
        )
        .await
    }
    pub async fn change_parent_of_note(
        change_parent_of_note_model: ChangeParentOfNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        let command_model =
            CommandModel::ChangeParentOfNoteCommandModel(change_parent_of_note_model);
        let event_model = Self::command_to_event(command_model);
        //Ok(Box::new(warp::reply::json(&customer)))
        let _res_eventstore = eventstore_service
            .append_to_stream(&eventstore_service.client, &user_id, &event_model)
            .await;
        let json = serde_json::to_string(&event_model).expect("Serde Error!");
        let _res = kafka_service.produce_message(&json, user_id);
        return Ok(Box::new(warp::reply::json(&event_model)));
    }

    pub async fn change_basic_info_note_handler(
        change_basic_info_note_model: ChangeBasicInfoNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        Self::change_basic_info_note(
            change_basic_info_note_model,
            user_id,
            eventstore_service,
            kafka_service,
        )
        .await
    }
    pub async fn change_basic_info_note(
        change_basic_info_note_model: ChangeBasicInfoNoteCommandModel,
        user_id: String,
        eventstore_service: EventstoreService,
        kafka_service: KafkaService,
    ) -> Result<Box<dyn warp::Reply>, Infallible> {
        let command_model =
            CommandModel::ChangeBasicInfoNoteCommandModel(change_basic_info_note_model);
        let event_model = Self::command_to_event(command_model);
        //Ok(Box::new(warp::reply::json(&customer)))
        let _res_eventstore = eventstore_service
            .append_to_stream(&eventstore_service.client, &user_id, &event_model)
            .await;
        let json = serde_json::to_string(&event_model).expect("Serde Error!");
        let _res = kafka_service.produce_message(&json, user_id);
        return Ok(Box::new(warp::reply::json(&event_model)));
    }

    fn with_eventstore_service(
        eventstore_service: EventstoreService,
    ) -> impl Filter<Extract = (EventstoreService,), Error = Infallible> + Clone {
        warp::any().map(move || eventstore_service.clone())
    }

    fn with_kafka_service(
        kafka_service: KafkaService,
    ) -> impl Filter<Extract = (KafkaService,), Error = Infallible> + Clone {
        warp::any().map(move || kafka_service.clone())
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

            CommandModel::ChangeParentOfNoteCommandModel(change_parent_of_note_model) => {
                EventModel::ParentOfNoteChangedEventModel(ParentOfNoteChangedEventModel {
                    event_id: Uuid::new_v4(),
                    event_timestamp: Utc::now(),
                    id: change_parent_of_note_model.id,
                    pid: change_parent_of_note_model.pid,
                })
            }
            CommandModel::ChangeBasicInfoNoteCommandModel(change_basic_info_note_model) => {
                EventModel::BasicInfoOfNoteChangedEventModel(BasicInfoOfNoteChangedEventModel {
                    event_id: Uuid::new_v4(),
                    event_timestamp: Utc::now(),
                    id: change_basic_info_note_model.id,
                    text: change_basic_info_note_model.text,
                    title: change_basic_info_note_model.title,
                })
            }
        }
    }
}
