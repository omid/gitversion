FROM rust:1-slim

RUN apt-get update\
    && apt-get install -y pkg-config --no-install-recommends\
    && apt-get install -y libssl-dev --no-install-recommends\
    && rm -rf /var/lib/apt/lists/*

RUN mkdir /repo
WORKDIR /repo

RUN cargo install gitversion
