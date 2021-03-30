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

fn create_address_update() -> items::AddressUpdate {
    items::AddressUpdate{
        country: String::from("Germany"),
        persons: vec![String::from("vla_pak")],
        addresses: vec![items::Address {
            street: String::from("Somewhere 12"),
            city: String::from("Keulen"),
        }],
    }
}

# [tokio::main]
async fn main() {
    let person = create_person();
    let adress_update = create_address_update();
    println!("person: {:?}", person);
    println!("adress update: {:?}", adress_update);
    let mut buf_person: Vec<u8> = Vec::with_capacity(person.encoded_len());
    person.encode(&mut buf_person).ok();
    println!("person: {:?}", buf_person);
    let mut buf_address_update: Vec<u8> = Vec::with_capacity(adress_update.encoded_len());
    adress_update.encode(&mut buf_address_update).ok();
    println!("adress update: {:?}", buf_address_update);
    let mut producer = get_producer("127.0.0.1:9092", String::from("http://localhost:8081"));
    producer.send_proto(person.id.as_bytes().to_vec(), buf_person, Name::Person).await;
    producer.send_proto(adress_update.country.as_bytes().to_vec(), buf_address_update, Name::AdressUpdate).await;
}
