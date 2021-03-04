FROM rust:latest as builder
COPY Cargo.toml Cargo.lock ./
COPY ./src ./src
RUN cargo install --path .
 
FROM ubuntu:latest
COPY --from=builder /usr/local/cargo/bin/trashbot /usr/bin/trashbot
RUN apt update -y && apt install openssl ca-certificates -y
RUN update-ca-certificates
