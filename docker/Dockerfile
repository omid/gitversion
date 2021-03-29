FROM rust:1 as builder

RUN apt update && apt install upx -y

RUN rustc --version

ADD . /app
WORKDIR /app

RUN cargo build --release

RUN ls -l /app/target/release/gitversion
RUN strip /app/target/release/gitversion
RUN upx /app/target/release/gitversion
RUN ls -l /app/target/release/gitversion

###

FROM frolvlad/alpine-glibc

RUN mkdir /repo
WORKDIR /repo

COPY --from=builder /app/target/release/gitversion /usr/bin/gitversion
