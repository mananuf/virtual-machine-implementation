# Define the main executable name (adjust as needed)
TARGET = virtual_machine

# Default target: builds the release version
all: build-release

cli:
	cargo run --bin cli

trace:
	RUST_LOG=$(TARGET)=trace cargo run --bin cli

info:
	RUST_LOG=$(TARGET)=info cargo run --bin cli

debug:
	RUST_LOG=$(TARGET)=debug cargo run --bin cli
	
# Target to build the debug version
build-debug:
	cargo build

# Target to build the release version
build-release:
	cargo build --release

# Target to run tests
test:
	cargo test

# Target to clean the project
clean:
	cargo clean

# Target to run the application (debug version)
run-debug: build-debug
	./target/debug/$(TARGET)

# Target to run the application (release version)
run-release: build-release
	./target/release/$(TARGET)

# Phony targets to prevent conflicts with files of the same name
.PHONY: all build-debug build-release test clean run-debug run-release