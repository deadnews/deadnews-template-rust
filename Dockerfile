FROM rust:1.91.1-alpine@sha256:8efbfb788786eeb127adc581394349c5fb567712156e0f8c2e499acadbc23756 AS builder

ARG PROFILE=release
ENV CARGO_HOME="/cache/cargo"

WORKDIR /app

RUN --mount=type=cache,target="/var/cache/apk" \
    apk add musl-dev

COPY --parents Cargo.toml Cargo.lock src/ ./
RUN --mount=type=cache,target=${CARGO_HOME} \
    cargo build --locked --profile ${PROFILE}

FROM gcr.io/distroless/static-debian12@sha256:4b2a093ef4649bccd586625090a3c668b254cfe180dee54f4c94f3e9bd7e381e AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ARG PROFILE=release
ENV SERVICE_PORT=8000

COPY --from=ghcr.io/tarampampam/microcheck:1.3.0@sha256:79c187c05bfa67518078bf4db117771942fa8fe107dc79a905861c75ddf28dfa /bin/httpcheck /bin/httpcheck

COPY --from=builder /app/target/${PROFILE}/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
