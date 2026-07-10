# Build stage
FROM rust:1.88-slim-trixie AS build
LABEL org.opencontainers.image.source="https://github.com/FAZuH/fazuh-site"

# Install system dependencies
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev curl ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Install Tailwind CSS standalone binary
RUN curl -fsSL --retry 3 --retry-delay 2 \
    https://github.com/tailwindlabs/tailwindcss/releases/latest/download/tailwindcss-linux-x64 \
    -o /usr/local/bin/tailwindcss && \
    chmod +x /usr/local/bin/tailwindcss

# Install Dioxus CLI
RUN rustup target add wasm32-unknown-unknown && \
    curl -fsSL --retry 3 --retry-delay 2 \
    https://github.com/DioxusLabs/dioxus/releases/download/v0.7.6/dx-x86_64-unknown-linux-gnu.tar.gz \
    -o /tmp/dx.tar.gz && \
    tar -xzf /tmp/dx.tar.gz -C /usr/local/bin dx && \
    chmod +x /usr/local/bin/dx && \
    rm /tmp/dx.tar.gz

WORKDIR /app

# Cache build dependencies
COPY Cargo.toml Cargo.lock ./
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    mkdir -p src/components src/utils src/bin && \
    echo "fn main() {}" > src/main.rs && \
    echo "" > src/app.rs && \
    echo "" > src/server.rs && \
    echo "" > src/utils.rs && \
    echo "" > src/components/mod.rs && \
    echo "" > src/utils/config.rs && \
    echo "" > src/utils/logging.rs && \
    echo "" > src/utils/rate_limit.rs && \
    echo "" > src/utils/validation.rs && \
    echo "" > src/utils/smtp.rs && \
    cargo build --release 2>/dev/null || true && \
    rm -rf src

COPY src/ ./src/
COPY assets/ ./assets/
COPY input.css ./

# Build app
RUN --mount=type=cache,target=/usr/local/cargo/registry,sharing=locked \
    --mount=type=cache,target=/usr/local/cargo/git,sharing=locked \
    tailwindcss -i input.css -o assets/tailwind.css && \
    dx build --release

# Runtime stage
FROM debian:trixie-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates && rm -rf /var/lib/apt/lists/*
RUN addgroup -S app && adduser -S -G app -h /app -s /sbin/nologin app

COPY --from=build /app/target/dx/fazuh-site/release/web/public /app/public
COPY --from=build /app/target/dx/fazuh-site/release/web/server /app/server

RUN mkdir /app/logs && chown -R app:app /app

# Dioxus binds to localhost inside the container by default;
# override via the IP env var to expose it on the container's network.
ENV IP=0.0.0.0
ENV PORT=8080

USER app
WORKDIR /app
CMD ["./server"]
