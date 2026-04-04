FROM rust:1.94.1-alpine@sha256:7f752ee8ea5deb9f4863d8c3f228a216a6466619882f09a44b9eda9617dc7770 AS builder

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

FROM gcr.io/distroless/static@sha256:47b2d72ff90843eb8a768b5c2f89b40741843b639d065b9b937b07cd59b479c6 AS runtime

COPY --from=ghcr.io/tarampampam/microcheck:1.3.0@sha256:79c187c05bfa67518078bf4db117771942fa8fe107dc79a905861c75ddf28dfa /bin/httpcheck /bin/httpcheck

COPY --from=builder /bin/template-rust /bin/template-rust

USER nonroot:nonroot
HEALTHCHECK NONE
EXPOSE 8000

ENTRYPOINT ["/bin/template-rust"]
