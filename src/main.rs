use std::error::Error;

use futures::future::join_all;
use log::info;
use simple_logger::SimpleLogger;
use tokio;

use config::KafkaConfig;

use crate::kafka::receive_messages;

mod config;
mod kafka;
mod billiards;
mod database;


#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let client_uri = "mongodb://localhost:27017";
    let client = mongodb::Client::with_uri_str(client_uri.as_ref()).await?;

    // Print the databases in our MongoDB cluster:

    println!("Databases:");
    for name in client.list_database_names(None, None).await? {
        println!("- {}", name);
    }

    let cfg = KafkaConfig::new();
    println!("{:#?}", cfg);

    SimpleLogger::new()
        .with_level(cfg.log_level.to_level_filter())
        .init().expect("Could not init logger");

    info!("Logging initialized with level {}", cfg.log_level);


    let futures =
        (0..cfg.parallel_operations)
            .map(|_| {
                tokio::spawn(
                    receive_messages(cfg.clone())
                )
            });
    join_all(futures).await;

    Ok(())
}
