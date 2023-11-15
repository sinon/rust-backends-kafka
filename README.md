# Kafka producers/consumers

### producer-api

Axum based API which receives events via HTTP API and publishes them to kafka topic

### consumer

Simple consumer which reads from the same topic as `producer-api` and echos the message to console

### Local 

Run zookeeper, kafka and create needed topic:
```bash
~/dev/kafka_2.13-2.7.0/bin/zookeeper-server-start.sh ~/dev/kafka_2.13-2.7.0/config/zookeeper.properties

~/dev/kafka_2.13-2.7.0/bin/kafka-server-start.sh ~/dev/kafka_2.13-2.7.0/config/server.properties

bin/kafka-topics.sh --create --topic user-behaviour.events --replication-factor 1 --partitions 2 --zookeeper localhost:2181

RUST_LOG=api=trace,tower_http=trace cargo run --bin producer-api

cargo run --bin consumer
```

Sample Request:
```
curl --request POST \
  --url http://localhost:3000/event \
  --header 'Content-Type: application/json' \
  --header 'User-Agent: insomnia/2023.5.8' \
  --data '{
  "data": {
    "subscription": "projects/test-project/subscriptions/my-subscription",
    "message": {
      "attributes": {
        "attr1": "attr1-value"
      },
      "data": "dGVzdCBtZXNzYWdlIDM=",
      "messageId": "message-id",
      "publishTime": "2021-02-05T04:06:14.109Z",
      "orderingKey": "ordering-key"
    }
  },
  "datacontenttype": "application/json",
  "id": "3103425958877813",
  "source": "//pubsub.googleapis.com/projects/test-project/topics/my-topic",
  "specversion": "1.0",
  "time": "2021-02-05T04:06:14.109Z",
  "type": "google.cloud.pubsub.topic.v1.messagePublished"
}'
```