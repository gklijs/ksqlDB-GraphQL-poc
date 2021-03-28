# ksqlDB GraphQL poc

Setup to serve as proof of concept in using Kafka with ksqlDB in combination with the query language GraphQL.

No materialized views have been registered. Register materialized views in order to use pull queries. Refer to https://cnfl.io/queries for info on query types.

`js-graphql-endpoint` draait op [http://localhost:4000](http://localhost:4000)

Open interactive ksql promt: `docker exec -it ksqldb ksql http://ksqldb:8088`

todo in ksqldb

` CREATE STREAM persons (id STRING KEY) WITH (KAFKA_TOPIC = 'persons', VALUE_FORMAT = 'PROTOBUF');`

`CREATE STREAM groups (id STRING KEY) WITH (KAFKA_TOPIC = 'groups', VALUE_FORMAT = 'PROTOBUF');`

Then some query to combine them, but:

```
CREATE STREAM groups_expanded AS
>SELECT
>groups.id
>FROM groups
>LEFT JOIN persons ON groups.members = persons.id
>EMIT CHANGES;
```

Is not working because `Invalid join. Key types differ: ARRAY<STRING> vs STRING`