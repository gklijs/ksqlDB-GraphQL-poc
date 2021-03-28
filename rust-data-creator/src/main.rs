extern crate prost;
use crate::prost::Message;
use crate::kafka_producer::{get_producer, Name};

mod items {
    include!(concat!(env!("OUT_DIR"), "/tech.gklijs.kgpoc.rs"));
}
mod kafka_producer;

fn create_person() -> items::Person {
    items::Person {
        id: String::from("vla_pak"),
        first_name: String::from("vla"),
        last_name: String::from("pak"),
        birthday: Some(items::Date{
            year: 1983,
            month: 8,
            day: 23,
        })
    }
}

fn create_group() -> items::Group {
    items::Group{
        id: String::from("germans"),
        members: vec![String::from("vla_pak")],
    }
}

# [tokio::main]
async fn main() {
    let person = create_person();
    let group = create_group();
    println!("person: {:?}", person);
    println!("group: {:?}", group);
    let mut buf_person: Vec<u8> = Vec::with_capacity(person.encoded_len());
    person.encode(&mut buf_person).ok();
    println!("person: {:?}", buf_person);
    let mut buf_group: Vec<u8> = Vec::with_capacity(group.encoded_len());
    group.encode(&mut buf_group).ok();
    println!("group: {:?}", buf_group);
    let mut producer = get_producer("127.0.0.1:9092", String::from("http://localhost:8081"));
    producer.send_proto(person.id.as_bytes().to_vec(), buf_person, Name::Person).await;
    producer.send_proto(group.id.as_bytes().to_vec(), buf_group, Name::Group).await;
}
