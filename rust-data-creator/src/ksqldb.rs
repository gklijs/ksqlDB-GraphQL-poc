use ksqldb::{KsqlDB, Error};
use reqwest::Client;
use std::env;

pub async fn create_streams_and_tables() {
    let url = env::var("KSQL_URL").unwrap_or(String::from("localhost:8088"));
    let ksql = KsqlDB::new(url, Client::builder(), false).unwrap();
    let query = r#"
    CREATE TABLE IF NOT EXISTS persons (id_key STRING PRIMARY KEY) WITH (KAFKA_TOPIC = 'persons', VALUE_FORMAT = 'PROTOBUF');
    CREATE STREAM IF NOT EXISTS address_updates (country_id STRING KEY) WITH (KAFKA_TOPIC = 'address-updates', VALUE_FORMAT = 'PROTOBUF');
    CREATE STREAM IF NOT EXISTS exploded_address_updates AS SELECT country_id, country, EXPLODE(persons) AS person_id, EXPLODE(addresses)
    AS address FROM address_updates;
    CREATE STREAM IF NOT EXISTS persons_with_address AS SELECT persons.id_key AS id, persons.first_name as first_name, persons.last_name
    as last_name, persons.birthday as birthday, country, address FROM exploded_address_updates JOIN persons ON
    exploded_address_updates.person_id = persons.id_key;
    "#;
    match ksql.create(query, &Default::default(), None).await {
        Ok(r) => println!("Success creating table and streams: {:?}", r),
        Err(e) => match e {
            Error::KSQL(k) => {
                if k.error_code == Some(40001){
                    println!("Assume all the things are already created properly, error was: {}", k)
                } else {
                    panic!("unhandled ksql error: {:?}", k)
                }
            },
            _ => panic!("unhandled error: {:?}", e)
        }
    }
}