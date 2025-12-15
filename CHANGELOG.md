# Changelog

## [1.2.2-alpha.1](https://github.com/deadnews/deadnews-template-rust/compare/v1.2.1...v1.2.2-alpha.1) - 2025-12-15

### Chores

- _(prek)_ add `zizmor` hook - ([37b7a24](https://github.com/deadnews/deadnews-template-rust/commit/37b7a242a16d99be2ac22cc5a0dad2fbde850a6c))

## [1.2.1](https://github.com/deadnews/deadnews-template-rust/compare/v1.2.0...v1.2.1) - 2025-12-02

### Chores

- _(config)_ migrate Renovate config ([#116](https://github.com/deadnews/deadnews-template-rust/issues/116)) - ([3968f05](https://github.com/deadnews/deadnews-template-rust/commit/3968f05a5c32b4f7356e2b928f6c749020484ea9))
- _(github)_ fix sed command for setting version in `cargo.toml` ([#117](https://github.com/deadnews/deadnews-template-rust/issues/117)) - ([e338346](https://github.com/deadnews/deadnews-template-rust/commit/e338346749bbbaa3b399e3653421022a00c85cbd))
- _(renovate)_ fix config - ([29a2dbb](https://github.com/deadnews/deadnews-template-rust/commit/29a2dbb6d5851697517b7b2fdb23a4e05152547e))

## [1.2.0](https://github.com/deadnews/deadnews-template-rust/compare/v1.1.0...v1.2.0) - 2025-12-02

### Features

- update project structure ([#114](https://github.com/deadnews/deadnews-template-rust/issues/114)) - ([bdad033](https://github.com/deadnews/deadnews-template-rust/commit/bdad0334dbf1e1b00a2100585cc9951abb6714a8))

### Bug fixes

- migrate from 2021 edition to 2024 - ([0a9b518](https://github.com/deadnews/deadnews-template-rust/commit/0a9b518a90e45ae9a87c881d690894fb1c6678a3))

### Documentation

- _(readme)_ remove static version - ([8bae487](https://github.com/deadnews/deadnews-template-rust/commit/8bae487d2f953a716eb6cb6af707bf7dcffd2986))

### Chores

- _(git-cliff)_ update config - ([816c3b2](https://github.com/deadnews/deadnews-template-rust/commit/816c3b2281f984a0944865e0b851ab3722a00f7f))
- _(github)_ add code coverage support ([#91](https://github.com/deadnews/deadnews-template-rust/issues/91)) - ([e483977](https://github.com/deadnews/deadnews-template-rust/commit/e48397735b5137eee78f430c8ef24a36e6e952ac))
- standardize repository references ([#92](https://github.com/deadnews/deadnews-template-rust/issues/92)) - ([b2fccfb](https://github.com/deadnews/deadnews-template-rust/commit/b2fccfb9804b03d3e833e52d2f23741f3e120a2e))

### Dependencies

- update cargo dependencies ([#105](https://github.com/deadnews/deadnews-template-rust/issues/105)) - ([d0a8112](https://github.com/deadnews/deadnews-template-rust/commit/d0a8112a29f2cc72ff592b520406d86afedb36f6))

## [1.1.0](https://github.com/deadnews/deadnews-template-rust/compare/v1.0.0...v1.1.0) - 2025-06-30

### Features

- add database connection handling ([#90](https://github.com/deadnews/deadnews-template-rust/issues/90)) - ([98ea130](https://github.com/deadnews/deadnews-template-rust/commit/98ea1302cbee87ed49925e666c56ae66b61ac573))

## [1.0.0](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.6...v1.0.0) - 2025-06-27

### Build

- _(docker)_ add `healthcheck` ([#56](https://github.com/deadnews/deadnews-template-rust/issues/56)) - ([cc59f78](https://github.com/deadnews/deadnews-template-rust/commit/cc59f7806b1ce73b55d326c89ab8a18f02e7b219))
- add `goreleaser` ([#86](https://github.com/deadnews/deadnews-template-rust/issues/86)) - ([7dd719a](https://github.com/deadnews/deadnews-template-rust/commit/7dd719ace50db963ee76e802deb32dbcd30b4e8c))

### Chores

- _(config)_ migrate renovate config ([#67](https://github.com/deadnews/deadnews-template-rust/issues/67)) - ([c32a6f3](https://github.com/deadnews/deadnews-template-rust/commit/c32a6f3afefc7042a7914543c7f8bbff8e9bffb5))
- _(github)_ remove `macos` from tests matrix - ([c0cea57](https://github.com/deadnews/deadnews-template-rust/commit/c0cea57c61f46b53bc251d3f2210a9859cb3b6d4))
- _(github)_ add `oldstable` to matrix ([#57](https://github.com/deadnews/deadnews-template-rust/issues/57)) - ([faad4ae](https://github.com/deadnews/deadnews-template-rust/commit/faad4ae208cf20e5808ea8ee81e064c2ecbdb3e3))

### Dependencies

- update rust crate log to v0.4.27 ([#80](https://github.com/deadnews/deadnews-template-rust/issues/80)) - ([7bd4f37](https://github.com/deadnews/deadnews-template-rust/commit/7bd4f3771be1f8bb6dc9bedbc9a74d9f5762de4e))

## [0.1.6](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.5...v0.1.6) - 2024-08-14

### Chores

- _(github)_ update workflows ([#54](https://github.com/deadnews/deadnews-template-rust/issues/54)) - ([37ce7ec](https://github.com/deadnews/deadnews-template-rust/commit/37ce7ece616a56a1273d7bf9e3046ee9acb2c34f))

## [0.1.5](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.4...v0.1.5) - 2024-08-14

### Documentation

- _(changelog)_ update git-cliff config - ([5c3be00](https://github.com/deadnews/deadnews-template-rust/commit/5c3be00243add491a1c14b02f9d85176c4caf6e3))
- _(changelog)_ add `git-cliff` ([#38](https://github.com/deadnews/deadnews-template-rust/issues/38)) - ([10f7513](https://github.com/deadnews/deadnews-template-rust/commit/10f75132678adf867e671551eeaf9e1e37585c5e))

### Chores

- _(github)_ use dynamic versioning ([#53](https://github.com/deadnews/deadnews-template-rust/issues/53)) - ([e4779cf](https://github.com/deadnews/deadnews-template-rust/commit/e4779cf53dde22e2ad1f9cf086324584526b394b))
- _(github)_ update `deadnews-template-docker` to `main` branch - ([dc0e276](https://github.com/deadnews/deadnews-template-rust/commit/dc0e276269848f36f48d7241e663252c15df6845))
- _(typos)_ ignore short words - ([2ca0b40](https://github.com/deadnews/deadnews-template-rust/commit/2ca0b40f5eebaae5618fc0b28776eb40e85cf826))
- _(vscode)_ replace `crates` with `dependi` - ([4ee173c](https://github.com/deadnews/deadnews-template-rust/commit/4ee173c414574278eafe3f2c57056d3e8ea2be89))

### Dependencies

- update rust crate actix-web to v4.8.0 ([#48](https://github.com/deadnews/deadnews-template-rust/issues/48)) - ([e328e45](https://github.com/deadnews/deadnews-template-rust/commit/e328e45b56058ed77f732ded9f3152b2e6a58018))

## [0.1.4](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.3...v0.1.4) - 2024-03-22

### Bug fixes

- bump version ([#32](https://github.com/deadnews/deadnews-template-rust/issues/32)) - ([5d247c8](https://github.com/deadnews/deadnews-template-rust/commit/5d247c8150179ce1ac34984fb910e463eac30a02))

### Chores

- _(dockerfile)_ fix `hadolint` warnings - ([e1d29c4](https://github.com/deadnews/deadnews-template-rust/commit/e1d29c45181a0ecf18b73102cd09885df0fb68a8))
- _(pre-commit)_ replace `hadolint-docker` with `hadolint-py` - ([d1e93fe](https://github.com/deadnews/deadnews-template-rust/commit/d1e93fefb4b74f046456094edc3921c9aa6be528))

### Dependencies

- update rust crate log to 0.4.21 ([#28](https://github.com/deadnews/deadnews-template-rust/issues/28)) - ([7e50a3f](https://github.com/deadnews/deadnews-template-rust/commit/7e50a3f5d39d8a2dfa497fab0e9cd2026a1db94d))

## [0.1.3](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.2...v0.1.3) - 2024-02-23

### Build

- _(docker)_ update `Dockerfile` - ([66ba247](https://github.com/deadnews/deadnews-template-rust/commit/66ba247ef5a2e2c911e4786f4f1629b8561ceff2))
- _(docker)_ add `docker-compose` - ([692cce5](https://github.com/deadnews/deadnews-template-rust/commit/692cce568fc2c3aad65d243d637765ac43f9b987))
- _(docker)_ update `Dockerfile` - ([512ae01](https://github.com/deadnews/deadnews-template-rust/commit/512ae01ce8c34191c262ddafb604d080d16eb7c5))

### Chores

- _(pre-commit)_ add `checkmake` hook - ([364916d](https://github.com/deadnews/deadnews-template-rust/commit/364916d958560af10b45b8f09fa2dd8fe6fc5986))
- add `docker-static` build ([#25](https://github.com/deadnews/deadnews-template-rust/issues/25)) - ([14339d6](https://github.com/deadnews/deadnews-template-rust/commit/14339d6d98ee8e6db4f8f69b8c8fc3d1a8a0be93))

### Dependencies

- update rust crate env_logger to 0.11.1 ([#23](https://github.com/deadnews/deadnews-template-rust/issues/23)) - ([de180ee](https://github.com/deadnews/deadnews-template-rust/commit/de180ee1930fac69de6d6cdc2ef36b6b667ce438))

## [0.1.2](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.1...v0.1.2) - 2023-10-01

### Chores

- bump version - ([dd504e5](https://github.com/deadnews/deadnews-template-rust/commit/dd504e5032e92161750f7bfae23d24cfd2297e85))

## [0.1.1](https://github.com/deadnews/deadnews-template-rust/compare/v0.1.0...v0.1.1) - 2023-10-01

### Chores

- bump version - ([7073184](https://github.com/deadnews/deadnews-template-rust/commit/7073184ba89e6ec15736a0e550faef833b57af5a))
- use `crates.io` deploy environment - ([d14a96f](https://github.com/deadnews/deadnews-template-rust/commit/d14a96f2ab8eb577277d7d966a1ad22551efa66e))
- set explicit permissions - ([525debd](https://github.com/deadnews/deadnews-template-rust/commit/525debdfd6a5c53fc7e6f6b4eeeb74f7549b10c0))
- update `cd` workflow - ([b53fa90](https://github.com/deadnews/deadnews-template-rust/commit/b53fa90afb9b51d9261330f4f1cd2934ca5f8293))

## [0.1.0](https://github.com/deadnews/deadnews-template-rust/commits/v0.1.0) - 2023-09-24

### Features

- init ([#2](https://github.com/deadnews/deadnews-template-rust/issues/2)) - ([f861c7b](https://github.com/deadnews/deadnews-template-rust/commit/f861c7b7bbf4787a50043ac338d307151e63063e))
- init - ([d3cd58a](https://github.com/deadnews/deadnews-template-rust/commit/d3cd58aab7f8f5b0571b11514a28a39de732753e))

### Chores

- update `cd` workflow - ([0a8695c](https://github.com/deadnews/deadnews-template-rust/commit/0a8695c89c99bd67060db04cd4a67ce5c475fab1))
- update workflows ([#4](https://github.com/deadnews/deadnews-template-rust/issues/4)) - ([e29aae6](https://github.com/deadnews/deadnews-template-rust/commit/e29aae6dace722d8a7d16d4e25c75209d9a5a0d7))

<!-- generated by git-cliff -->
