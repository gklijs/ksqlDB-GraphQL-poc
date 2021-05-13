# ksqlDB GraphQL poc

Setup to serve as proof of concept in using Kafka with ksqlDB in combination with the query language GraphQL.

`js-graphql-endpoint` runs on [http://localhost:4000](http://localhost:4000) (once it can handle references)

Open interactive ksql promt: `docker exec -it ksqldb ksql http://ksqldb:8088`

The `data-producer` is updating every 5 seconds, which causes all the persons in the update to update.

See the results: `kafka-protobuf-console-consumer --topic PERSONS_WITH_ADDRESS --bootstrap-server localhost:9092 --from-beginning`