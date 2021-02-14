### Start message bus and game-events topic

* reference: [Kafka quick start](https://kafka.apache.org/quickstart)
1. Open terminal window and start the zookeeper service
  ```
  cd ~/kafka/kafka_2.13-2.7.0/
  bin/zookeeper-server-start.sh config/zookeeper.properties
  ```
2. In a new terminal tab, start the broker service
  ```
  bin/kafka-server-start.sh config/server.properties
  ```
3. In a new terminal tab, create a game-events topic and publish event messages
  ```    
  bin/kafka-topics.sh --create --topic game-events --bootstrap-server localhost:9092  
  ```
4. Publish event messages
  ```    
  bin/kafka-console-producer.sh --topic game-events --bootstrap-server localhost:9092
  
  --input event messages at prompt
  ```    



