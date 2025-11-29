FROM rust:1.91.1-slim@sha256:5218a2b4b4cb172f26503ac2b2de8e5ffd629ae1c0d885aff2cbe97fd4d1a409 AS builder

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

FROM gcr.io/distroless/static-debian12@sha256:87bce11be0af225e4ca761c40babb06d6d559f5767fbf7dc3c47f0f1a466b92c AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ENV SERVICE_PORT=8000

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
