use log::Level;

#[derive(Debug, Clone)]
pub struct KafkaConfig {
    pub kafka_input_topic: String,
    pub kafka_brokers: String,
    pub kafka_consumer_group: String,
    pub log_level: Level,
    pub parallel_operations: usize,
}

impl KafkaConfig {
    pub fn new() -> Self {
        Self {
            kafka_input_topic : "game-events".to_string(),
            kafka_brokers : "127.0.0.1:9092".to_string(),
            kafka_consumer_group : "game-events-group".to_string(),
            log_level : Level::Info,
            parallel_operations: 5
        }
    }
}
