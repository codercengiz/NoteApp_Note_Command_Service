use std::{convert::Infallible, net::SocketAddr};
use warp::reject::Reject;
use warp::{
    http::{HeaderValue, StatusCode},
    hyper::HeaderMap,
};
use warp::{Filter, Rejection, Reply};

use crate::{settings::WebServerSettings};
use crate::models::CreateNoteCommandModel;


pub(crate) struct WebServer {
    pub socket_addr: SocketAddr,
}

impl WebServer {
    pub(crate) fn init(settings: WebServerSettings) -> Self {
        WebServer {
            socket_addr: settings.socket_addr,
        }
    }
    pub(crate) async fn start(self) {
        let routes = self
        .add_note_route()
        .with(warp::cors().allow_any_origin()
        .allow_methods(vec!["POST"]))
        .with(warp::log("webserver"));
        warp::serve(routes).run(self.socket_addr).await;
    }

    /// POST /add-note
    fn add_note_route(&self) -> impl Filter<Extract = impl Reply, Error = Rejection> + Clone {
        
        warp::path!("add-note")
            .and(warp::post())
            .and(warp::body::json())
            .and(warp::header::<String>("user-id"))
            .and_then(Self::add_note_handler)
            
    }
    pub async fn add_note_handler(create_note_model:CreateNoteCommandModel, user_id:String ) -> Result<Box<dyn warp::Reply>, Infallible> {
        
        Ok(Box::new(StatusCode::NOT_FOUND))
    }

    
}
