FROM rust:1.51 as builder

ARG PROTOC_ZIP=protoc-3.15.6-linux-x86_64.zip
RUN curl -OL https://github.com/protocolbuffers/protobuf/releases/download/v3.15.6/${PROTOC_ZIP} &&\
    unzip -o ${PROTOC_ZIP} -d /usr/local bin/protoc &&\
    unzip -o ${PROTOC_ZIP} -d /usr/local 'include/*' &&\
    rm ${PROTOC_ZIP}
RUN apt-get update &&\
    apt-get install -y build-essential &&\
    apt-get install -y cmake
WORKDIR /workdir
COPY ./ ./
RUN cargo build --release
RUN mkdir -p /build-out
RUN cp target/release/rust-data-creator /build-out/

FROM ubuntu:groovy-20210325

COPY --from=builder /build-out/rust-data-creator /
CMD /rust-data-creator