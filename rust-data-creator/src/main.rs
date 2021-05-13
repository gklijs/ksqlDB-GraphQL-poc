use crate::ksqldb::create_streams_and_tables;
use crate::data_producer::create_data;
use tokio::time::sleep;
use tokio::time;

mod ksqldb;
mod data_producer;

# [tokio::main]
async fn main() {
    create_streams_and_tables().await;
    sleep(time::Duration::from_secs(10)).await;
    create_data().await;
}
