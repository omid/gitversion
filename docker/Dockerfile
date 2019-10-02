FROM rust:1 as builder

RUN cargo install gitversion

###

FROM frolvlad/alpine-glibc

RUN mkdir /repo
WORKDIR /repo

COPY --from=builder /usr/local/cargo/bin/gitversion /usr/bin/gitversion
