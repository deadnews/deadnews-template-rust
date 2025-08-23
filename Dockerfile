FROM rust:1.89.0-slim@sha256:6c828d9865870a3bc8c02919d73803df22cac59b583d8f2cb30a296abe64748f AS builder

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

FROM gcr.io/distroless/static-debian12@sha256:2e114d20aa6371fd271f854aa3d6b2b7d2e70e797bb3ea44fb677afec60db22c AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ENV SERVICE_PORT=8000

COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
