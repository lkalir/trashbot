FROM rust:latest as builder
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo install --path .
 
FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/trashbot /usr/bin/trashbot
RUN apt update -y && apt install libssl1.1 -y
