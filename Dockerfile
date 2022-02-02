# Use rust nightly as we use some feature flags
FROM rustlang/rust:nightly as build

# Create a new package and move into the dir
RUN USER=root cargo new --bin starlight
WORKDIR /starlight

# Install needed deps
RUN apt-get update && apt-get install -y cmake build-essential git clang libssl-dev libxxhash-dev zlib1g-dev pkg-config

# Install mold as a linker
COPY ./mold-install.sh ./mold-install.sh
RUN ./mold-install.sh

# Copy everything because we have subcrates within the main crate
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
COPY ./rust-toolchain.toml ./rust-toolchain.toml
COPY ./.cargo/config.toml ./.cargo/config.toml
COPY ./starlight-macros ./starlight-macros


# Build the empty ./src, which contains the default main.rs from cargo new
RUN cargo build --release

# Remove the empty source and add ours, to prevent rebuilding of deps on every change
RUN rm -rf src/
COPY ./src ./src

# Remove old build, and rebuild
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
COPY --from=deps /usr/bin/dumb-init /usr/bin/dumb-init

ENTRYPOINT ["/usr/bin/dumb-init"]

CMD ["./starlight"]
