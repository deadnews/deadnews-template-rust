FROM rust:1.94.0-alpine@sha256:ef7b340d4201444fa2757dfddfd4c03be9d2bde468de7b7a68b0e9fabb794334 AS builder

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

FROM gcr.io/distroless/static-debian13@sha256:28efbe90d0b2f2a3ee465cc5b44f3f2cf5533514cf4d51447a977a5dc8e526d0 AS runtime

COPY --from=ghcr.io/tarampampam/microcheck:1.3.0@sha256:79c187c05bfa67518078bf4db117771942fa8fe107dc79a905861c75ddf28dfa /bin/httpcheck /bin/httpcheck

COPY --from=builder /bin/template-rust /bin/template-rust

USER nonroot:nonroot
HEALTHCHECK NONE
EXPOSE 8000

ENTRYPOINT ["/bin/template-rust"]
