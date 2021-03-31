# ksqlDB GraphQL poc

Setup to serve as proof of concept in using Kafka with ksqlDB in combination with the query language GraphQL.

No materialized views have been registered. Register materialized views in order to use pull queries. Refer to https://cnfl.io/queries for info on query types.

`js-graphql-endpoint` draait op [http://localhost:4000](http://localhost:4000)

Open interactive ksql promt: `docker exec -it ksqldb ksql http://ksqldb:8088`