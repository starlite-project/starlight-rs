# Use rust nightly as we use some feature flags
FROM rustlang/rust:nightly-buster-slim as build

# Create a new package and move into the dir
RUN USER=root cargo new --bin starlight
WORKDIR /starlight

RUN apt-get update && apt-get install -y cmake

# Copy everything because we have subcrates within the main crate
COPY ./.cargo ./.cargo
COPY ./nebula ./nebula
COPY ./star-derive ./star-derive
COPY ./star-test ./star-test
COPY ./supernova ./supernova
COPY ./build.rs ./build.rs
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml
# Replace our actual main with an empty one, to build deps
RUN cargo build --release

# Remove the empty source and add ours, to prevent rebuilding of deps on every change
RUN rm -rf src/
COPY ./src ./src
RUN rm ./target/release/deps/starlight*
RUN cargo build --release
RUN strip -s ./target/release/starlight

# Download certs from an alpine image
FROM alpine:3.6 as deps

RUN apk add -U --no-cache ca-certificates dumb-init

# Use slim image for final build
FROM debian:buster-slim
COPY --from=build /starlight/target/release/starlight .
COPY --from=deps /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=deps /usr/bin/dumb-init .
# RUN apt-get update && apt-get install -y ca-certificates

CMD ["./dumb-init", "./starlight"]
