###### To start Kafka

- web link reference: https://kafka.apache.org/quickstart
* Open terminal window and start the zookeeper service
    - cd ~/kafka/kafka_2.13-2.7.0/
    - bin/zookeeper-server-start.sh config/zookeeper.properties
* start the broker service
    - Open new terminal tab
    - bin/kafka-server-start.sh config/server.properties
* create a game-events topic and publish event messages
    - Open new terminal tab
    - bin/kafka-topics.sh --create --topic game-events --bootstrap-server localhost:9092  
    - bin/kafka-console-producer.sh --topic game-events --bootstrap-server localhost:9092
    



