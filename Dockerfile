FROM rust:alpine as builder
RUN apk add --no-cache build-base

WORKDIR /usr/src/unimap
COPY Cargo.toml Cargo.lock ./
COPY src/ src/
RUN cargo install --path .

FROM alpine:3.12

RUN apk add --no-cache nmap nmap-scripts wget
COPY --from=builder /usr/local/cargo/bin/unimap /usr/local/bin/unimap

ENTRYPOINT [ "/usr/local/bin/unimap" ]