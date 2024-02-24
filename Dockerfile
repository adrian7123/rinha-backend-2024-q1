FROM rust:1.76.0-slim-buster AS build

RUN cargo new --bin app

WORKDIR /app

COPY Cargo.toml /app/
COPY Cargo.lock /app/
COPY Rocket.toml /app/

COPY src /app/src
COPY prisma /app/prisma
COPY prisma-cli /app/prisma-cli
COPY prisma.sh /app/

RUN apt update && apt upgrade -y
RUN apt install pkg-config libssl-dev -y

RUN chmod +x prisma.sh

RUN touch /app/src/main.rs
RUN bash prisma.sh generate
RUN cargo build --release

CMD "/app/target/release/rinha-backend-2024-q1"