.PHONY: all clean default run build install check lint pc test release

default: check

run:
	cargo run

build:
	cargo build

goreleaser:
	goreleaser --clean --snapshot --skip=publish

install:
	prek install

update:
	cargo update --recursive --verbose
	prek auto-update

check: pc lint test
pc:
	prek run -a
lint:
	cargo fmt --all
	cargo clippy --fix --allow-dirty --all-features -- -D warnings
lint-ci:
	cargo fmt --all --check
	cargo clippy --all-targets --all-features -- -D warnings
test:
	cargo test --all-features

test-cov:
	cargo llvm-cov --ignore-filename-regex 'test.rs'
	cargo llvm-cov report --lcov --output-path lcov.info

test-codecov:
	cargo llvm-cov --ignore-filename-regex 'test.rs' --codecov --output-path codecov.json

doc:
	cargo doc --no-deps --document-private-items --all-features

bumped:
	git cliff --bumped-version

# make release TAG=$(git cliff --bumped-version)-alpha.0
release: check
	git cliff -o CHANGELOG.md --tag $(TAG)
	prek run --files CHANGELOG.md || prek run --files CHANGELOG.md
	git add CHANGELOG.md
	git commit -m "chore(release): prepare for $(TAG)"
	git push
	git tag -a $(TAG) -m "chore(release): $(TAG)"
	git push origin $(TAG)
