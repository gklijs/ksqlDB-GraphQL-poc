extern crate prost;

use crate::data_producer::kafka_producer::{get_producer, Name, RecordProducer};
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

fn get_country(nr: i32) -> String {
    match nr % 4 {
        0 => String::from("Germany"),
        1 => String::from("Netherlands"),
        2 => String::from("Belgium"),
        _ => String::from("Sweden"),
    }
}

fn get_persons(nr: i32) -> Vec<String> {
    let mut persons = vec![];
    let modifier = nr % 13;
    if nr % 3 == 0 {
        persons.push(format!("id_{}", modifier))
    }
    if nr % 4 == 0 {
        persons.push(format!("id_{}", 1 + modifier))
    }
    if nr % 5 == 0 {
        persons.push(format!("id_{}", 2 + modifier))
    }
    if nr % 6 == 0 {
        persons.push(format!("id_{}", 3 + modifier))
    }
    if nr % 7 == 0 {
        persons.push(format!("id_{}", 4 + modifier))
    }
    if nr % 8 == 0 {
        persons.push(format!("id_{}", 5 + modifier))
    }
    if nr % 9 == 0 {
        persons.push(format!("id_{}", 6 + modifier))
    }
    if persons.len() < 2 {
        persons.push(format!("id_{}", 7 + modifier))
    }
    persons
}

fn get_city(nr: i32) -> String {
    match nr % 12 {
        0 => String::from("Berlin"),
        1 => String::from("Amsterdam"),
        2 => String::from("Antwerp"),
        3 => String::from("Stockholm"),
        4 => String::from("Bonn"),
        5 => String::from("Utrecht"),
        6 => String::from("Brussels"),
        7 => String::from("Gothenburg"),
        8 => String::from("Hamburg"),
        9 => String::from("Rotterdam"),
        10 => String::from("Bruges"),
        _ => String::from("Norje"),
    }
}

fn get_addresses(nr: i32, len: usize) -> Vec<items::Address> {
    let city = get_city(nr);
    let mut addresses = vec![];
    for i in 0..len {
        addresses.push(items::Address {
            street: format!("Some Street {}", nr % 233 + i as i32 + 1),
            city: city.clone(),
        })
    }
    addresses
}

fn create_address_update(nr: i32) -> items::AddressUpdate {
    let persons = get_persons(nr);
    let persons_len = persons.len();
    items::AddressUpdate {
        country: get_country(nr),
        persons,
        addresses: get_addresses(nr, persons_len),
    }
}

async fn send_person(mut producer: RecordProducer<'_>, nr: i32) -> RecordProducer<'_> {
    let person = create_person(nr);
    let mut buf_person: Vec<u8> = Vec::with_capacity(person.encoded_len());
    person.encode(&mut buf_person).ok();
    producer
        .send_proto(person.id.as_bytes().to_vec(), buf_person, Name::Person)
        .await;
    println!("person for nr: {}, was added.", nr);
    producer
}

async fn send_address_update(mut producer: RecordProducer<'_>, nr: i32) -> RecordProducer<'_> {
    let address_update = create_address_update(nr);
    let mut buf_address_update: Vec<u8> = Vec::with_capacity(address_update.encoded_len());
    address_update.encode(&mut buf_address_update).ok();
    producer
        .send_proto(
            address_update.country.as_bytes().to_vec(),
            buf_address_update,
            Name::AdressUpdate,
        )
        .await;
    println!("address_update for nr: {}, was added.", nr);
    producer
}

pub async fn create_data() {
    let brokers = env::var("KAFKA_BROKERS").unwrap_or_else(|_| String::from("localhost:9092"));
    let schema_registry_url =
        env::var("SCHEMA_REGISTRY_URL").unwrap_or_else(|_| String::from("http://localhost:8081"));
    let mut producer = get_producer(&*brokers, schema_registry_url);
    for nr in 0..20 {
        producer = send_person(producer, nr).await
    }
    for nr in 0..10000 {
        sleep(time::Duration::from_secs(5)).await;
        producer = send_address_update(producer, nr).await
    }
}
