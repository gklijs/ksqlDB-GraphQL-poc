use crate::ksqldb::create_streams_and_tables;
use crate::data_producer::set_data;

mod ksqldb;
mod data_producer;

# [tokio::main]
async fn main() {
    create_streams_and_tables().await;
    set_data().await;
}
