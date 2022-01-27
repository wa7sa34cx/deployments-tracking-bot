# Builder stage
FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 as chef
WORKDIR /app

FROM chef as planner
COPY . .
# Compute a lock-like file for project
RUN cargo chef prepare  --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
# Build project dependencies, not the application itself
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
# Build the project
RUN cargo build --release

# Runtime stage
FROM debian:bullseye-slim AS runtime
WORKDIR /app
# Install OpenSSL
RUN apt-get update -y \
    && apt-get install -y --no-install-recommends openssl \
    # Clean up
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/deployments-tracking-bot deployments-tracking-bot
COPY /app/messages messages
# Don't forget about environment variables!
ENTRYPOINT ["./deployments-tracking-bot"]
