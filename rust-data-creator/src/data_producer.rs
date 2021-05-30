extern crate prost;

use crate::data_producer::kafka_producer::{get_producer, RecordProducer};
use crate::data_producer::prost::Message;
use std::env;
use tokio::time;
use tokio::time::sleep;

mod items {
    include!(concat!(env!("OUT_DIR"), "/tech.gklijs.kgpoc.rs"));
}

mod kafka_producer;

fn create_person(nr: i32) -> items::Person {
    items::Person {
        id: format!("id_{}", nr),
        first_name: format!("first_{}", nr),
        last_name: format!("last_{}", nr),
        birthday: Some(items::Date {
            year: 1950 + nr % 65,
            month: 1 + nr % 12,
            day: 1 + nr % 28,
        }),
    }
}

async fn send_person(producer: &RecordProducer, nr: i32) {
    let person = create_person(nr);
    let mut buf_person: Vec<u8> = Vec::with_capacity(person.encoded_len());
    person.encode(&mut buf_person).ok();
    producer
        .send_person(person.id.as_bytes().to_vec(), buf_person)
        .await;
    println!("person for nr: {}, was added.", nr);
}

pub async fn create_data() {
    let brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| String::from("localhost:9092"));
    let schema_registry_url =
        env::var("SCHEMA_REGISTRY_URL").unwrap_or_else(|_| String::from("http://localhost:8081"));
    let producer = get_producer(&*brokers, schema_registry_url);
    for nr in 0..10000 {
        send_person(&producer, nr).await;
        sleep(time::Duration::from_secs(5)).await;
    }
}
