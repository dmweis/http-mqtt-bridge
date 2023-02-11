# base builder
FROM rust:latest as chef

WORKDIR /app
RUN apt-get update && apt-get install lld clang -y
RUN cargo install cargo-chef

# rust layer caching
FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# rebuild if dependencies changed
FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin http-mqtt-bridge

# light runtime
FROM debian:bullseye-slim AS runtime

WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl ca-certificates \
    # Clean up so runtime is light
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/http-mqtt-bridge http-mqtt-bridge
COPY configuration/settings.yaml settings.yaml
CMD ["--config","settings.yaml"]
ENTRYPOINT ["./http-mqtt-bridge"]
