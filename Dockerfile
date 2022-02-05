# Use a base image and install latest nightly
FROM rustlang/rust:nightly as build

RUN USER=root cargo new --bin starlight
WORKDIR /starlight

RUN apt-get update && apt-get install -y cmake clang

# Copy everything because we have subcrates within the main crate
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml
COPY ./.cargo/config.toml ./.cargo/config.toml
COPY ./starlight-macros ./starlight-macros

RUN rustup component add rust-src

# Build the empty ./src, which contains the default main.rs from cargo new
RUN cargo build --release -Z build-std=std,panic_abort --target x86_64-unknown-linux-gnu

# Remove the empty source and add ours, to prevent rebuilding of deps on every change
RUN rm -rf src/
COPY ./src ./src

# Remove old build, and rebuild
RUN rm ./target/x86_64-unknown-linux-gnu/release/deps/starlight*
RUN cargo build --release -Z build-std=std,panic_abort --target x86_64-unknown-linux-gnu
RUN strip -s ./target/x86_64-unknown-linux-gnu/release/starlight

# Download certs from an alpine image
FROM alpine:3.6 as deps

RUN apk add -U --no-cache ca-certificates dumb-init

# Use slim image for final build
FROM debian:buster-slim
COPY --from=build starlight/target/x86_64-unknown-linux-gnu/release/starlight .
COPY --from=deps /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=deps /usr/bin/dumb-init /usr/bin/dumb-init

ENTRYPOINT ["/usr/bin/dumb-init"]

CMD ["./starlight"]
