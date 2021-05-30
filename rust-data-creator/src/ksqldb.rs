use ksqldb::{Error, KsqlDB};
use reqwest::Client;
use std::env;

pub async fn create_streams_and_tables() {
    let url = env::var("KSQL_URL").unwrap_or_else(|_| String::from("localhost:8088"));
    let ksql = KsqlDB::new(url, Client::builder(), false).unwrap();
    let query = r#"
    CREATE TABLE IF NOT EXISTS persons (id_key STRING PRIMARY KEY) WITH (KAFKA_TOPIC = 'persons', VALUE_FORMAT = 'PROTOBUF');
    CREATE TABLE IF NOT EXISTS persons_by_birthyear AS SELECT BIRTHDAY->YEAR as BIRTHYEAR, COUNT(*) as TOTAL FROM persons GROUP BY BIRTHDAY->YEAR EMIT CHANGES;
    "#;
    match ksql.create(query, &Default::default(), None).await {
        Ok(r) => println!("Success creating table and streams: {:?}", r),
        Err(e) => match e {
            Error::KSQL(k) => {
                if k.error_code == Some(40001) {
                    println!(
                        "Assume all the things are already created properly, error was: {}",
                        k
                    )
                } else {
                    panic!("unhandled ksql error: {:?}", k)
                }
            }
            _ => panic!("unhandled error: {:?}", e),
        },
    }
}
