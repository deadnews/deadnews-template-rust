# deadnews-template-rust

> Rust Project Template

[![Rust: Crates.io](https://img.shields.io/badge/dynamic/json?url=https://crates.io/api/v1/crates/deadnews-template-rust&query=$.crate.max_stable_version&prefix=v&label=crates.io&logo=rust&logoColor=white&color=orange)](https://crates.io/crates/deadnews-template-rust)
[![PyPI: Version](https://img.shields.io/pypi/v/deadnews-template-rust?logo=pypi&logoColor=white)](https://pypi.org/project/deadnews-template-rust)
[![GitHub: Release](https://img.shields.io/github/v/release/deadnews/deadnews-template-rust?logo=github&logoColor=white)](https://github.com/deadnews/deadnews-template-rust/releases/latest)
[![Docker: ghcr](https://img.shields.io/badge/docker-gray.svg?logo=docker&logoColor=white)](https://github.com/deadnews/deadnews-template-rust/pkgs/container/deadnews-template-rust)
[![CI: Main](https://img.shields.io/github/actions/workflow/status/deadnews/deadnews-template-rust/main.yml?branch=main&logo=github&logoColor=white&label=main)](https://github.com/deadnews/deadnews-template-rust/actions/workflows/main.yml)
[![CI: Coverage](https://img.shields.io/endpoint?url=https://raw.githubusercontent.com/deadnews/deadnews-template-rust/refs/heads/badges/coverage.json)](https://github.com/deadnews/ideadnews-template-rust)

## Installation

Docker

```sh
docker pull ghcr.io/deadnews/deadnews-template-rust
```

PyPI

```sh
uv tool install deadnews-template-rust
```

Cargo

```sh
cargo install deadnews-template-rust
```

## Endpoints

### GET /health

Health check endpoint.

```sh
curl -X GET http://127.0.0.1:8000/health
```

### GET /test

Returns database name and version as JSON.

```sh
curl -X GET http://127.0.0.1:8000/test
```
