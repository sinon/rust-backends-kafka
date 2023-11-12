# kafka-event-api

Axum based API which receives events via HTTP API and publishes them to kafka topic

### Local 

Run zookeeper, kafka and create needed topic:
```bash
~/dev/kafka_2.13-2.7.0/bin/zookeeper-server-start.sh ~/dev/kafka_2.13-2.7.0/config/zookeeper.properties

~/dev/kafka_2.13-2.7.0/bin/kafka-server-start.sh ~/dev/kafka_2.13-2.7.0/config/server.properties

bin/kafka-topics.sh --create --topic user-behaviour.events --replication-factor 1 --partitions 2 --zookeeper localhost:2181
```