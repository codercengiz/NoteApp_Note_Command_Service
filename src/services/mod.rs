mod web_server;
mod eventstore_service;

use crate::settings::Settings;
use crate::services::eventstore_service::EventstoreService;
use self::{ web_server::WebServer};

pub(crate) async fn run(settings: Settings) {
    let eventstore_service = EventstoreService::init(settings.eventstore_settings);
    let server = WebServer::init(settings.web_server_settings,eventstore_service.await);

    server.start().await;
}
