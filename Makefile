.PHONY: all build run release clean check fmt lint watch help

# default target
all: build

# build debug
build:
	cargo build

# run in debug mode
run:
	cargo run

# build optimized release binary
release:
	cargo build --release

# run release binary directly
run-release: release
	./target/release/system_monitor

# check for compile errors without building
check:
	cargo check

# format code
fmt:
	cargo fmt

# run clippy linter
lint:
	cargo clippy -- -D warnings

# clean build artifacts
clean:
	cargo clean

# watch for changes and auto-rerun (requires cargo-watch)
watch:
	cargo watch -x run

# show help
help:
	@echo ""
	@echo "  system_monitor - available commands"
	@echo ""
	@echo "  make build        build debug binary"
	@echo "  make run          build and run (debug)"
	@echo "  make release      build optimized release binary"
	@echo "  make run-release  build and run release binary"
	@echo "  make check        check for compile errors"
	@echo "  make fmt          format source code"
	@echo "  make lint         run clippy linter"
	@echo "  make clean        remove build artifacts"
	@echo "  make watch        auto-rerun on file changes"
	@echo ""