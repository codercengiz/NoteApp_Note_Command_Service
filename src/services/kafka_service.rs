extern crate kafka;

use std::time::Duration;

use kafka::error::Error as KafkaError;
use kafka::producer::{Producer, Record, RequiredAcks};

use crate::settings::KafkaSettings;

pub(crate) struct KafkaService {
    pub broker: String,
    pub topic: String,
}
impl KafkaService {
    pub fn init(settings: KafkaSettings) -> Self {
        KafkaService {
            broker: settings.broker,
            topic: settings.producer_topics[0].to_string(),
        }
    }

    pub fn produce_message(&self, data: &str, key:String) -> Result<(), KafkaError> {
        info!(
            "About to publish a message at {:?} to: {}",
            &self.broker, &self.topic
        );

        let data = data.as_bytes();
        let brokers = vec![self.broker.to_owned()];
        let mut producer = Producer::from_hosts(brokers)
            .with_ack_timeout(Duration::from_secs(1))
            .with_required_acks(RequiredAcks::One)
            .create()?;

        producer.send(&Record {
            topic: &self.topic,
            partition: -1,
            key,
            value: data,
        })

        
    }
}

impl Clone for KafkaService {
    fn clone(&self) -> Self {
        KafkaService {
            broker: self.broker.clone(),
            topic: self.topic.clone(),
        }
    }
}