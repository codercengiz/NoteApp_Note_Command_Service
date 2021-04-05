mod web_server;
mod eventstore_service;
mod kafka_service;

use crate::settings::Settings;
use crate::services::eventstore_service::EventstoreService;
use crate::services::kafka_service::KafkaService;
use self::{ web_server::WebServer};

pub(crate) async fn run(settings: Settings) {
    let eventstore_service = EventstoreService::init(settings.eventstore_settings).await;
    let kafka_service = KafkaService::init(settings.kafka_settings);
    let server = WebServer::init(settings.web_server_settings,eventstore_service, kafka_service);

    server.start().await;
}
