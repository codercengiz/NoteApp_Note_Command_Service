mod web_server;

use crate::settings::Settings;

use self::web_server::WebServer;

pub(crate) async fn run(settings: Settings) {
    
    let server = WebServer::init(settings.web_api_settings);
    

    server.start();
}
