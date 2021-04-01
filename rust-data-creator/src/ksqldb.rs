use ksqldb::KsqlDB;
use reqwest::Client;

pub async fn create_streams_and_tables() {
    let ksql = KsqlDB::new("localhost:8088".into(), Client::builder(), false).unwrap();
    let query = r#"
    CREATE TABLE persons (id_key STRING PRIMARY KEY) WITH (KAFKA_TOPIC = 'persons', VALUE_FORMAT = 'PROTOBUF');
    CREATE STREAM address_updates (country_id STRING KEY) WITH (KAFKA_TOPIC = 'address-updates', VALUE_FORMAT = 'PROTOBUF');
    CREATE STREAM exploded_address_updates AS SELECT country_id, country, EXPLODE(persons) AS person_id, EXPLODE(addresses)
    AS address FROM address_updates;
    CREATE STREAM persons_with_address AS SELECT persons.id_key AS id, persons.first_name as first_name, persons.last_name
    as last_name, persons.birthday as birthday, country, address FROM exploded_address_updates JOIN persons ON
    exploded_address_updates.person_id = persons.id_key;
    "#;
    ksql.create(query, &Default::default(), None).await.unwrap();
}