FROM rust:1.76.0-slim-buster AS build

RUN cargo new --bin app

WORKDIR /app

COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY Rocket.toml /app/

COPY src /app/src

RUN apt update && apt upgrade -y
RUN apt install pkg-config libssl-dev -y

RUN touch /app/src/main.rs
RUN cargo build --release

CMD "/app/target/release/rinha-backend-2024-q1"