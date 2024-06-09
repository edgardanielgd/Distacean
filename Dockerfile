FROM rust:1.78-bullseye AS builder

WORKDIR /app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

WORKDIR /app

COPY --from=builder /app/target/release/distacean .

CMD ["./distacean"]