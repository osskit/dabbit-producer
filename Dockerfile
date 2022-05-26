FROM rust:1.61 as builder
WORKDIR /usr/src/service

RUN USER=root cargo init --bin .
COPY ./Cargo.toml ./Cargo.lock ./
RUN cargo build --release && rm src/*.rs && rm target/release/deps/dabbit_producer*
COPY ./src ./src
RUN cargo install --path .

FROM debian:buster-slim
COPY --from=builder /usr/local/cargo/bin/dabbit-producer /usr/local/bin/service
CMD ["service"]