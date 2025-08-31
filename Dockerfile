FROM rust:1.89.0-slim@sha256:e556a015ecb064ca6b3b74bceb36a54deaf88afbe2956b8fe3e445da446d9cf8 AS builder

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

FROM gcr.io/distroless/static-debian12@sha256:f2ff10a709b0fd153997059b698ada702e4870745b6077eff03a5f4850ca91b6 AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ENV SERVICE_PORT=8000

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
