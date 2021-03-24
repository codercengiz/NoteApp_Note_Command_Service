use clap::{App, ArgMatches};

use std::fmt::Debug;
use std::net::SocketAddr;

pub(crate) struct Settings {
    pub web_api_settings: WebServerSettings,
    pub kafka_settings: KafkaSettings,
    pub eventstore_settings: EventstoreSettings,
}

impl Settings {
    pub(crate) fn init() -> Settings {
        let yaml = load_yaml!("app_settings.yaml");
        let matches = App::from(yaml).get_matches();
        Settings {
            web_api_settings: Self::get_web_api_settings(&matches),
            kafka_settings: Self::get_kafka_settings(&matches),
            eventstore_settings: Self::get_eventstore_settings(&matches),
        }
    }

    fn get_web_api_settings(matches: &ArgMatches) -> WebServerSettings {
        let domain = matches
            .value_of("domain")
            .expect("Invalid Domain")
            .to_string();
        let port = matches.value_of("port").expect("Invalid Port").to_string();
        let addr_string = format!("{}:{}", domain, port);
        let socket_addr = addr_string
            .parse()
            .expect("Invalid Socket Address");

        WebServerSettings { socket_addr }
    }

    fn get_kafka_settings(matches: &ArgMatches) -> KafkaSettings {
        KafkaSettings {
            brokers: matches
                
                .value_of("kafka-brokers-addr")
                .expect("Invalid Brokers")
                .to_string(),

            topics: matches
                .values_of("kafka-topics")
                .expect("Invalid Topics")
                .map(|x| x.to_string())
                .collect(),
        }
    }

    fn get_eventstore_settings(matches: &ArgMatches) -> EventstoreSettings {
        EventstoreSettings {
            socket_addr: matches
                .value_of("eventstore-socket-addr")
                .expect("Invalid eventstore addr")
                .parse()
                .expect("Unable to parse socket address"),

            username: matches
                .value_of("eventstore-username")
                .expect("Invalid Eventstore Username")
                .to_string(),
            password: matches
                .value_of("eventstore-password")
                .expect("Invalid Eventstore Password")
                .to_string(),
        }
    }
}

#[derive(Debug)]
pub(crate) struct WebServerSettings {
    pub(crate) socket_addr: SocketAddr,
}

#[derive(Debug)]
pub(crate) struct KafkaSettings {
    pub(crate) brokers: String,
    pub(crate) topics: Vec<String>,
}

#[derive(Debug)]
pub(crate) struct EventstoreSettings {
    pub(crate) socket_addr: SocketAddr,
    pub(crate) username: String,
    pub(crate) password: String,
}
