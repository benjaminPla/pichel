FROM rust:slim-bookworm AS chef

RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
RUN cargo install cargo-chef --locked

WORKDIR /app

# --- planner: dependency recipe from Cargo.toml + Cargo.lock
FROM chef AS planner

COPY api/ .
RUN cargo chef prepare --recipe-path recipe.json

# --- builder: deps layer stays cached until the recipe changes
FROM chef AS builder

COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY api/ .
COPY db/migrations/ /db/migrations/

RUN cargo build --release

# --- runtime
FROM debian:bookworm-slim AS runtime

RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

COPY --from=builder /app/target/release/api .

EXPOSE 3000

CMD ["./api"]
