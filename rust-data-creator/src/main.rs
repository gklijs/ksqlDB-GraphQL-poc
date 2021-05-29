use crate::data_producer::create_data;
use crate::ksqldb::create_streams_and_tables;
use tokio::time;
use tokio::time::sleep;

mod data_producer;
mod ksqldb;

#[tokio::main]
async fn main() {
    create_streams_and_tables().await;
    sleep(time::Duration::from_secs(10)).await;
    create_data().await;
}
