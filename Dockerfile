FROM rust:1.96.1-alpine@sha256:a41f7740f8b45d45795624eec13a8b42263cc700f19f7e4e86e04d3dda08a479 AS builder

ARG PROFILE=release
ENV CARGO_HOME="/cache/cargo" \
    CARGO_TARGET_DIR="/cache/target"

WORKDIR /app

RUN --mount=type=cache,target="/var/cache/apk" \
    apk add musl-dev

COPY --parents Cargo.toml Cargo.lock src/ ./
RUN --mount=type=cache,target=${CARGO_HOME} \
    --mount=type=cache,target=${CARGO_TARGET_DIR} \
    cargo build --locked --profile ${PROFILE} && \
    cp ${CARGO_TARGET_DIR}/${PROFILE}/deadnews-template-rust /bin/template-rust

FROM gcr.io/distroless/static@sha256:3592aa8171c77482f62bbc4164e6a2d141c6122554ace66e5cc910cadb961ff0 AS runtime

COPY --from=ghcr.io/tarampampam/microcheck:1.4.0@sha256:c9f79cd408626de7c10f2d487d67339f49adf0ba61dde96ede65343269db1f85 /bin/httpcheck /bin/httpcheck

COPY --from=builder /bin/template-rust /bin/template-rust

USER nonroot:nonroot
HEALTHCHECK NONE
EXPOSE 8000

ENTRYPOINT ["/bin/template-rust"]
