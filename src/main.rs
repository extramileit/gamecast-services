use log::info;
use simple_logger::SimpleLogger;
use tokio::runtime::Runtime;
use futures::future::join_all;
use config::KafkaConfig;
use crate::kafka::receive_messages;

mod config;
mod kafka;
mod billiards;


fn main() {
    let cfg = KafkaConfig::new();
    println!("{:#?}", cfg);

    SimpleLogger::new()
        .with_level(cfg.log_level.to_level_filter())
        .init().expect("Could not init logger");

    info!("Logging initialized with level {}", cfg.log_level);

    Runtime::new().unwrap().block_on(async {
        let futures =
            (0..cfg.parallel_operations)
                .map(|_| {
                    tokio::spawn(
                        receive_messages(cfg.clone())
                    )
                });
        join_all(futures).await;
    });

}
