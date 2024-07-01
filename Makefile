# This is a Makefile for the rust project
# This Makefile is used to automate the build, run, test, lint, and format commands
# build: build the rust project
build:
	cargo build --release

# run the rust project
run:
	cargo run

# format the rust code
fmt:
	cargo fmt

# run the rust tests
test:
	cargo test

# run linting on rust code
lint:
	cargo clippy

# do all actions in one command
all:
	make fmt lint test build
