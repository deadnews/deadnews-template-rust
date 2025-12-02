FROM rust:1.91.1-alpine@sha256:fbcca3e30e26f79986809d5dbfcdbeaaf8d3f8a4475b7a19a973363b45c74d97 AS builder

ARG PROFILE=release
ENV CARGO_HOME="/cache/cargo"

WORKDIR /app

RUN --mount=type=cache,target="/var/cache/apk" \
    apk add musl-dev

COPY --parents Cargo.toml Cargo.lock src/ ./
RUN --mount=type=cache,target=${CARGO_HOME} \
    cargo build --locked --profile ${PROFILE}

FROM gcr.io/distroless/static-debian12@sha256:87bce11be0af225e4ca761c40babb06d6d559f5767fbf7dc3c47f0f1a466b92c AS runtime
LABEL maintainer="deadnews <deadnewsgit@gmail.com>"

ARG PROFILE=release
ENV SERVICE_PORT=8000

COPY --from=ghcr.io/tarampampam/microcheck:1.2.0@sha256:42cdee55eddc4c5b5bd76cb6df5334ca22ce7dc21066aeea7d059898ffbf84fd /bin/httpcheck /bin/httpcheck

COPY --from=builder /app/target/${PROFILE}/deadnews-template-rust /bin/template-rust

USER nonroot:nonroot
EXPOSE ${SERVICE_PORT}
HEALTHCHECK NONE

ENTRYPOINT ["/bin/template-rust"]
