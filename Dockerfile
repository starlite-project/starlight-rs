# Use rust nightly as we use some feature flags
FROM rustlang/rust:nightly-buster-slim as build

# Create a new package and move into the dir
RUN USER=root cargo new --bin starlight
WORKDIR /starlight

RUN apt-get update && apt-get install -y cmake

# Copy everything because we have subcrates within the main crate, and remove the main crate
COPY ./ ./
RUN rm -rf src/

# Replace our actual main with an empty one, to build deps
RUN mkdir src/
RUN echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release

# Remove the empty source and add ours, to prevent rebuilding of deps on every change
RUN rm -rf src/
COPY ./src ./src
RUN rm ./target/release/deps/starlight*
RUN cargo build --release
RUN strip -s ./target/release/starlight

# Download certs from an alpine image
FROM alpine:3.6 as certs

RUN apk add -U --no-cache ca-certificates

# Use slim image for final build
FROM debian:buster-slim
COPY --from=build /starlight/target/release/starlight .
COPY --from=certs /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
# RUN apt-get update && apt-get install -y ca-certificates

CMD ["./starlight"]
