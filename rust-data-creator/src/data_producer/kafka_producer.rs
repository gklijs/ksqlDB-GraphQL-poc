use rdkafka::config::ClientConfig;
use rdkafka::producer::{BaseRecord, DefaultProducerContext, ThreadedProducer};
use schema_registry_converter::async_impl::easy_proto_raw::EasyProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub struct RecordProducer {
    producer: ThreadedProducer<DefaultProducerContext>,
    proto_encoder: EasyProtoRawEncoder,
}

impl RecordProducer {
    pub async fn send_person(&self, key_bytes: Vec<u8>, value_bytes: Vec<u8>) {
        let value_strategy = SubjectNameStrategy::TopicNameStrategy(String::from("persons"), false);
        let payload = match self
            .proto_encoder
            .encode_single_message(&*value_bytes, value_strategy)
            .await
        {
            Ok(v) => v,
            Err(e) => panic!("Error getting payload: {}", e),
        };
        let br = BaseRecord::to("persons").payload(&payload).key(&key_bytes);
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
