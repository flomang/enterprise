# coinbase 
A coinbase pro market feed kafka producer.

Usage: coinbase [OPTIONS] --market <MARKET> --topic <TOPIC>

Options:
*  -m, --market <MARKET>  The market to connect to e.g. 'BTC=USD'
*  -b, --broker <BROKER>  Kafka broker defaults to 'localhost:9092' [default: localhost:9092]
*  -t, --topic <TOPIC>    Kafka topic
*  -h, --help             Print help
*  -V, --version          Print version

## Prequisites
* [kafka](https://kafka.apache.org/quickstart)

Install it:
```
brew install kafka
```

This will install:
* kafka-topics
* kafka-console-producer
* kafka-console-consumer

Start it:
```
brew services start zookeeper
brew services start kafka 
```

### Kafka 
You will need to ensure that the kafka topic is created before running the utility. 

Create topic:
```
kafka-topics --create --topic coinbase-events --bootstrap-server localhost:9092
```

### To interact with kafka using the kafka utils:

Produce some events:
```
kafka-console-producer --topic coinbase-events --bootstrap-server localhost:9092
> Ding!
> Dong!
```

Consume:
```
kafka-console-consumer --topic coinbase-events --from-beginning --bootstrap-server localhost:9092
```


## Building
```
cargo build
```

## Running

From the workspace:
```
cargo run -p coinbase -- --market BTC-USD -b localhost:9092 -t coinbase-events
```


From the project:
```
cargo run -- -m ETH-USD -b localhost:9092 -t coinbase-events
```

