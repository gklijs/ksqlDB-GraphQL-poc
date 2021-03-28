use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use schema_registry_converter::async_impl::proto_raw::ProtoRawEncoder;
use schema_registry_converter::async_impl::schema_registry::SrSettings;
use schema_registry_converter::schema_registry_common::SubjectNameStrategy;

pub struct RecordProducer<'a> {
    producer: FutureProducer,
    proto_encoder: ProtoRawEncoder<'a>,
}

pub enum Name{
    Person,
    Group,
}

fn get_full_name(name: &Name) -> &'static str{
    match name {
        Name::Person => "tech.gklijs.kgpoc.Person",
        Name::Group => "tech.gklijs.kgpoc.Group",
    }
}

fn get_topic(name: &Name) -> &'static str{
    match name {
        Name::Person => "persons",
        Name::Group => "groups",
    }
}

impl RecordProducer <'_>{
    pub async fn send_proto(
        &'_ mut self,
        key_bytes: Vec<u8>,
        value_bytes: Vec<u8>,
        name: Name,
    ) {
        let value_strategy= SubjectNameStrategy::TopicNameStrategy(String::from(get_topic(&name)), false);
        let payload = match self.proto_encoder.encode(&*value_bytes, get_full_name(&name), value_strategy).await {
            Ok(v) => v,
            Err(e) => panic!("Error getting payload: {}", e),
        };
        let fr = FutureRecord {
            topic: get_topic(&name),
            partition: None,
            payload: Some(&payload),
            key: Some(&key_bytes),
            timestamp: None,
            headers: None,
        };
        self.producer.send_result(fr).unwrap().await.unwrap().unwrap();
    }
}

pub fn get_producer(brokers: &str, schema_registry_url: String) -> RecordProducer {
    let producer: FutureProducer = ClientConfig::new()
        .set("bootstrap.servers", brokers)
        .set("produce.offset.report", "true")
        .set("message.timeout.ms", "60000")
        .set("queue.buffering.max.messages", "10")
        .create()
        .expect("Producer creation error");

    let sr_settings = SrSettings::new(schema_registry_url);
    let proto_encoder = ProtoRawEncoder::new(sr_settings);
    RecordProducer {
        producer,
        proto_encoder,
    }
}
