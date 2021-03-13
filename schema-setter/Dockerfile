FROM maven:3.6.3-openjdk-17-slim

COPY  . /root/app/
WORKDIR /root/app

ENTRYPOINT mvn -Dschema-registry-url=${SCHEMA_REGISTRY_URL} -o schema-registry:register