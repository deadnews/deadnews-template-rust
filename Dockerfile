FROM rust:1.88.0-slim@sha256:38bc5a86d998772d4aec2348656ed21438d20fcdce2795b56ca434cf21430d89 AS builder

ENV CARGO_HOME="/cache/cargo"

WORKDIR /app

COPY Cargo.toml Cargo.lock ./
COPY src/ ./src/

# Install musl-tools to cross-compile the application for a lean image.
RUN --mount=type=cache,target="/var/cache/" \
    --mount=type=cache,target="/var/lib/apt/lists/" \
    apt-get update && apt-get install -y --no-install-recommends musl-tools

# Build the application for the musl target.
RUN --mount=type=cache,target=${CARGO_HOME} \
    rustup target add x86_64-unknown-linux-musl && \
    cargo build --release --locked --target x86_64-unknown-linux-musl

FROM gcr.io/distroless/static-debian12@sha256:b7b9a6953e7bed6baaf37329331051d7bdc1b99c885f6dbeb72d75b1baad54f9 AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ENV SERVICE_PORT=8000

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
