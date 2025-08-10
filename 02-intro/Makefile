# Operating Systems: Three Easy Pieces - Introduction Programs
# Makefile for building Rust Cargo projects

# Default target
all: rust

# Rust targets using Cargo
rust: cpu mem threads io

cpu:
	@echo "Building CPU program..."
	cd rust-cpu && cargo build --release
	cp rust-cpu/target/release/rust-cpu ./cpu

mem:
	@echo "Building Memory program..."
	cd mem && cargo build --release
	cp mem/target/release/mem ./mem

threads:
	@echo "Building Threads program..."
	cd concurrency && cargo build --release
	cp concurrency/target/release/concurrency ./threads
	@if [ -f concurrency/thread ]; then cp concurrency/thread ./threads-alt; fi

io:
	@echo "Building I/O program..."
	cd io && cargo build --release
	cp io/target/release/io ./io

# Test targets
test: rust
	@echo "Testing Rust programs..."
	@echo "=== CPU Test ==="
	@timeout 3s ./cpu "Hello OS" || echo "CPU test completed (timeout expected)"
	@echo ""
	@echo "=== Memory Test ==="
	@timeout 3s ./mem 100 || echo "Memory test completed (timeout expected)"
	@echo ""
	@echo "=== Threads Test ==="
	@./threads 10000
	@echo ""
	@echo "=== I/O Test ==="
	@./io && echo "I/O test result:" && cat /tmp/file

# Demonstration targets
demo-race: threads
	@echo "Demonstrating race conditions..."
	@for i in 1 2 3; do echo "Run $$i:"; ./threads 100000; done

demo-memory: mem
	@echo "Demonstrating memory allocation..."
	@timeout 5s ./mem 1000 || echo "Memory demo completed"

# Development targets
dev: rust
	@echo "Running in development mode..."

check:
	@echo "Checking all Cargo projects..."
	cd rust-cpu && cargo check
	cd mem && cargo check
	cd concurrency && cargo check
	cd io && cargo check

format:
	@echo "Formatting all Rust projects..."
	cd rust-cpu && cargo fmt
	cd mem && cargo fmt
	cd concurrency && cargo fmt
	cd io && cargo fmt

lint:
	@echo "Linting all Rust projects..."
	cd rust-cpu && cargo clippy
	cd mem && cargo clippy
	cd concurrency && cargo clippy
	cd io && cargo clippy

# Clean targets
clean: clean-rust clean-temp clean-binaries

clean-rust:
	@echo "Cleaning Rust build artifacts..."
	cd rust-cpu && cargo clean
	cd mem && cargo clean
	cd concurrency && cargo clean
	cd io && cargo clean

clean-binaries:
	@echo "Removing copied binaries..."
	rm -f cpu mem threads threads-alt io

clean-temp:
	@echo "Cleaning temporary files..."
	rm -f /tmp/file

# Update dependencies
update:
	@echo "Updating dependencies..."
	cd rust-cpu && cargo update
	cd mem && cargo update
	cd concurrency && cargo update
	cd io && cargo update

# Build and run individual programs
run-cpu: cpu
	./cpu "Hello from CPU"

run-mem: mem
	./mem 42

run-threads: threads
	./threads 100000

run-io: io
	./io

# Install targets (copy to /usr/local/bin - requires sudo)
install: rust
	@echo "Installing binaries to /usr/local/bin..."
	sudo cp cpu /usr/local/bin/ostep-cpu
	sudo cp mem /usr/local/bin/ostep-mem
	sudo cp threads /usr/local/bin/ostep-threads
	sudo cp io /usr/local/bin/ostep-io

uninstall:
	@echo "Removing installed binaries..."
	sudo rm -f /usr/local/bin/ostep-{cpu,mem,threads,io}

# Help target
help:
	@echo "Available targets:"
	@echo "  all          - Build all Rust programs"
	@echo "  rust         - Build all Rust programs (same as all)"
	@echo "  cpu          - Build only CPU program"
	@echo "  mem          - Build only Memory program"
	@echo "  threads      - Build only Threads program"
	@echo "  io           - Build only I/O program"
	@echo "  test         - Run tests for all programs"
	@echo "  demo-race    - Demonstrate race conditions"
	@echo "  demo-memory  - Demonstrate memory program"
	@echo "  check        - Run cargo check on all projects"
	@echo "  format       - Format all Rust code"
	@echo "  lint         - Lint all Rust code"
	@echo "  clean        - Clean all build artifacts and binaries"
	@echo "  update       - Update Cargo dependencies"
	@echo "  run-cpu      - Build and run CPU program"
	@echo "  run-mem      - Build and run Memory program"
	@echo "  run-threads  - Build and run Threads program"
	@echo "  run-io       - Build and run I/O program"
	@echo "  install      - Install binaries to /usr/local/bin"
	@echo "  uninstall    - Remove installed binaries"
	@echo "  help         - Show this help message"

# Phony targets
.PHONY: all rust cpu mem threads io test demo-race demo-memory dev check format lint clean clean-rust clean-binaries clean-temp update run-cpu run-mem run-threads run-io install uninstall help