FROM rustlang/rust:nightly-buster-slim as build

RUN USER=root cargo new --bin starlight
WORKDIR /starlight

RUN apt-get update && apt-get install -y cmake
# Copy everything because we have subcrates within the main crate
COPY ./ ./
RUN rm src/*.rs

# Replace our actual main with an empty one
RUN echo 'fn main() {}' > ./src/main.rs
RUN cargo build --release

RUN rm src/*.rs

COPY ./src ./src
RUN rm ./target/release/deps/starlight*
RUN cargo build --release

# FROM rustlang/rust:nightly-buster-slim
FROM ubuntu
COPY --from=build /starlight/target/release/starlight .
RUN apt-get update && apt-get install -y ca-certificates

CMD ["./starlight"]
