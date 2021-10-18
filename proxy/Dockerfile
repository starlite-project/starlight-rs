FROM rustlang/rust:nightly as build

RUN USER=root cargo new --bin star-proxy
WORKDIR /star-proxy

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml


RUN cargo build --release
RUN rm -f ./src/*.rs

COPY . .

RUN rm -f ./target/release/deps/star-proxy*
RUN cargo build --release

FROM alpine:3.6 as certs

RUN apk add -U --no-cache ca-certificates

FROM debian:buster-slim
COPY --from=build /star-proxy/target/release/star-proxy .
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt

CMD ["./star-proxy"]