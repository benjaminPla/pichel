FROM rust:slim AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*

COPY api/ .
COPY db/migrations/ /db/migrations/

RUN cargo build --release

FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api .

EXPOSE 3000

CMD ["./api"]
