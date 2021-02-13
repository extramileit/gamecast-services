use crate::config::KafkaConfig;
use rdkafka::{ClientConfig, Message};
use rdkafka::consumer::{StreamConsumer, Consumer};
use rdkafka::config::RDKafkaLogLevel;
use log::*;
use rdkafka::message::{BorrowedMessage, FromBytes};
use futures_util::{TryStreamExt};

pub async fn receive_messages(cfg: KafkaConfig) {
    let consumer = create_consumer(&cfg.kafka_brokers, &cfg.kafka_consumer_group, &cfg.kafka_input_topic);

    let stream_processor = consumer.stream().try_for_each(|borrowed_message| {
       async move {
           record_borrowed_message_receipt(&borrowed_message).await;
           // let owned_message = borrowed_message.detach();
           Ok(())
       }
    });
    info!("Starting event loop");
    stream_processor.await.expect("stream processing failed");
    info!("Stream processing terminated");
}

pub fn create_consumer(brokers: &str, group_id: &str, topic: &str) -> StreamConsumer {
    let consumer: StreamConsumer = ClientConfig::new()
        .set("group.id", group_id)
        .set("bootstrap.servers", brokers)
        .set("enable.partition.eof", "false")
        .set("session.timeout.ms", "6000")
        .set("enable.auto.commit", "true")
        .set("auto.commit.interval.ms", "1000")
        .set("enable.auto.offset.store", "false")
        .set_log_level(RDKafkaLogLevel::Debug)
        .create()
        .expect("Consumer creation failed");

    consumer.subscribe(&[topic]).expect("Cannot subscribe to specified topic");
    consumer
}

async fn record_borrowed_message_receipt(msg: &BorrowedMessage<'_>) {
    // Simulate some work that must be done in the same order as messages are
    // received; i.e., before truly parallel processing can begin.
    info!("Message received: {}", msg.offset());
    let payload_str = str::from_bytes(msg.payload().unwrap()).unwrap();
    info!("Message contents: {}", payload_str);
}
