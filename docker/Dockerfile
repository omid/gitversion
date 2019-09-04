FROM rust:1 as builder

RUN cargo install gitversion

###

FROM debian:stable-slim

RUN apt update\
    && apt install -y libssl-dev libcurl4-openssl-dev --no-install-recommends\
    && rm -rf /var/lib/apt/lists/*

RUN mkdir /repo
WORKDIR /repo

COPY --from=builder /usr/local/cargo/bin/gitversion /usr/bin/gitversion
