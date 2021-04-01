use crate::ksqldb::create_streams_and_tables;
use crate::data_producer::set_data;
use std::{thread, time};

mod ksqldb;
mod data_producer;

# [tokio::main]
async fn main() {
    create_streams_and_tables().await;
    thread::sleep(time::Duration::from_secs(10));
    set_data().await;
}
