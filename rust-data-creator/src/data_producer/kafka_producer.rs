use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub struct RecordProducer {
    producer: ThreadedProducer<DefaultProducerContext>,
    proto_encoder: EasyProtoRawEncoder,
}

pub enum Name {
    Person,
    AdressUpdate,
}

fn get_topic(name: &Name) -> &'static str {
    match name {
        Name::Person => "persons",
        Name::AdressUpdate => "address-updates",
    }
}

impl RecordProducer {
    pub async fn send_proto(&self, key_bytes: Vec<u8>, value_bytes: Vec<u8>, name: Name) {
        let value_strategy =
            SubjectNameStrategy::TopicNameStrategy(String::from(get_topic(&name)), false);
        let payload = match self
            .proto_encoder
            .encode_single_message(&*value_bytes,  value_strategy)
            .await
        {
            Ok(v) => v,
            Err(e) => panic!("Error getting payload: {}", e),
        };
        let br = BaseRecord::to(get_topic(&name))
            .payload(&payload)
            .key(&key_bytes);
        self.producer.send(br).unwrap();
    }
}

pub fn get_producer(brokers: &str, schema_registry_url: String) -> RecordProducer {
    let producer: ThreadedProducer<DefaultProducerContext> = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "60000")
        .set("queue.buffering.max.messages", "10")
        .create()
        .expect("Producer creation error");

    let sr_settings = SrSettings::new(schema_registry_url);
    let proto_encoder = EasyProtoRawEncoder::new(sr_settings);
    RecordProducer {
        producer,
        proto_encoder,
    }
}
