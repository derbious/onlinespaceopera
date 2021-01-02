FROM rust:alpine as builder
RUN apk add rust
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM alpine:latest
WORKDIR /app
COPY --from=builder /usr/src/app/target/release/spaceopera spaceopera
COPY staticweb ./staticweb
CMD ["./spaceopera"]
