FROM rust:1.61-alpine3.16 as builder
RUN apk add sqlite-dev

WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.16
COPY --from=builder /src/target/release/rustube /usr/local/bin/rustube

VOLUME /config
WORKDIR /config

CMD ["rustube"]
