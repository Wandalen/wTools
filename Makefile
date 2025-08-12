
# This Makefile provides a leveled system for testing and watching a Rust project.
#

#
# === Parameters ===
#

# Defines package flags for cargo commands if a crate is specified.
# e.g., `make ctest1 crate=my-app` will set PKG_FLAGS to `-p my-app`.
PKG_FLAGS = $(if $(crate),-p $(crate))

#
# === .PHONY section ===
#

.PHONY : \
	help \
	env-install \
	env-check \
	cwa \
	ctest1 \
	ctest2 \
	ctest3 \
	ctest4 \
	ctest5 \
	wtest1 \
	wtest2 \
	wtest3 \
	wtest4 \
	wtest5 \
	clean-cache-files

#
# === Help ===
#

# Display the list of available commands.
#
# Usage:
#	make help
help:
	@echo "=== Rust Development Makefile Commands ==="
	@echo ""
	@echo "Setup:"
	@echo "  env-install      - Install all required development tools (cargo-nextest, willbe, etc.)."
	@echo "  env-check        - Manually verify that all required tools are installed."
	@echo ""
	@echo "Workspace Management:"
	@echo "  cwa              - Full update and clean workspace (rustup + cargo tools + cache cleanup)."
	@echo ""
	@echo "Test Commands (each level includes all previous steps):"
	@echo "  ctest1 [crate=..] - Level 1: Primary test suite (cargo nextest run)."
	@echo "  ctest2 [crate=..] - Level 2: Primary + Documentation tests."
	@echo "  ctest3 [crate=..] - Level 3: Primary + Doc + Linter checks."
	@echo "  ctest4 [crate=..] - Level 4: All checks + Heavy testing (unused deps + audit)."
	@echo "  ctest5 [crate=..] - Level 5: Full heavy testing with mutation tests."
	@echo ""
	@echo "Watch Commands (auto-run on file changes):"
	@echo "  wtest1 [crate=..] - Watch Level 1: Primary tests only."
	@echo "  wtest2 [crate=..] - Watch Level 2: Primary + Doc tests."
	@echo "  wtest3 [crate=..] - Watch Level 3: Primary + Doc + Linter."
	@echo "  wtest4 [crate=..] - Watch Level 4: All checks + Heavy testing (deps + audit)."
	@echo "  wtest5 [crate=..] - Watch Level 5: Full heavy testing with mutations."
	@echo ""
	@echo "Cache Management:"
	@echo "  clean-cache-files - Add hyphen prefix to cache files for git exclusion."
	@echo ""


#
# === Setup ===
#

# Install all tools for the development environment.
#
# Usage :
#	make env-install
env-install:
	@echo "Setting up nightly toolchain..."
	@rustup toolchain install nightly
	@echo "\nInstalling required development tools..."
	@cargo install cargo-nextest cargo-wipe cargo-watch willbe cargo-audit
	@cargo +nightly install cargo-udeps
	@echo "\nDevelopment environment setup is complete!"

# Manually verify that the development environment is installed correctly.
#
# Usage :
#	make env-check
env-check:
	@echo "Verifying development environment..."
	@rustup toolchain list | grep -q 'nightly' || (echo "Error: Rust nightly toolchain not found. Please run 'make env-install'" && exit 1)
	@command -v cargo-nextest >/dev/null || (echo "Error: cargo-nextest not found. Please run 'make env-install'" && exit 1)
	@command -v cargo-wipe >/dev/null || (echo "Error: cargo-wipe not found. Please run 'make env-install'" && exit 1)
	@command -v cargo-watch >/dev/null || (echo "Error: cargo-watch not found. Please run 'make env-install'" && exit 1)
	@command -v willbe >/dev/null || (echo "Error: willbe not found. Please run 'make env-install'" && exit 1)
	@command -v cargo-udeps >/dev/null || (echo "Error: cargo-udeps not found. Please run 'make env-install'" && exit 1)
	@command -v cargo-audit >/dev/null || (echo "Error: cargo-audit not found. Please run 'make env-install'" && exit 1)
	@echo "Environment verification successful."

#
# === Workspace Management ===
#

# Full update and clean workspace.
#
# Usage :
#	make cwa
cwa:
	@clear
	@echo "Running full workspace update and clean..."
	@rustup update
	@echo "\nUpdating cargo tools..."
	@cargo install -q cargo-update cargo-wipe cargo-cache
	@echo "\nCleaning cargo cache..."
	@cargo cache --autoclean-expensive --gc
	@echo "\nWiping build artifacts..."
	@cargo wipe rust
	@echo "\nWiping node modules..."
	@cargo wipe node
	@echo "\nWiping target directory..."
	@cargo wipe -w
	@echo "\nWorkspace update and clean complete."

#
# === Test Commands ===
#

# Test Level 1: Primary test suite.
#
# Usage :
#	make ctest1 [crate=name]
ctest1:
	@clear
	@echo "Running Test Level 1: Primary test suite..."
	@RUSTFLAGS="-D warnings" cargo nextest run $(PKG_FLAGS)

# Test Level 2: Primary + Documentation tests.
#
# Usage :
#	make ctest2 [crate=name]
ctest2:
	@clear
	@echo "Running Test Level 2: Primary + Doc tests..."
	@RUSTFLAGS="-D warnings" cargo nextest run $(PKG_FLAGS) && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc $(PKG_FLAGS)

# Test Level 3: Primary + Doc + Linter.
#
# Usage :
#	make ctest3 [crate=name]
ctest3:
	@clear
	@echo "Running Test Level 3: All standard checks..."
	@RUSTFLAGS="-D warnings" cargo nextest run $(PKG_FLAGS) && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc $(PKG_FLAGS) && \
	cargo clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings

# Test Level 4: All standard + Heavy testing (deps, audit).
#
# Usage :
#	make ctest4 [crate=name]
ctest4:
	@clear
	@echo "Running Test Level 4: All checks + Heavy testing..."
	@RUSTFLAGS="-D warnings" cargo nextest run $(PKG_FLAGS) && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc $(PKG_FLAGS) && \
	cargo clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings && \
	cargo +nightly udeps --all-targets $(PKG_FLAGS) && \
	cargo +nightly audit $(PKG_FLAGS) && \
	$(MAKE) --no-print-directory clean-cache-files

# Test Level 5: Full heavy testing with mutation tests.
#
# Usage :
#	make ctest5 [crate=name]
ctest5:
	@clear
	@echo "Running Test Level 5: Full heavy testing with mutations..."
	@RUSTFLAGS="-D warnings" cargo nextest run $(PKG_FLAGS) && \
	RUSTDOCFLAGS="-D warnings" cargo test --doc $(PKG_FLAGS) && \
	cargo clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings && \
	willbe .test dry:0 && \
	cargo +nightly udeps --all-targets $(PKG_FLAGS) && \
	cargo +nightly audit $(PKG_FLAGS) && \
	$(MAKE) --no-print-directory clean-cache-files

#
# === Watch Commands ===
#

# Watch Level 1: Primary tests only.
#
# Usage :
#	make wtest1 [crate=name]
wtest1:
	@echo "Watching Level 1: Primary tests..."
	@cargo watch -c -x "nextest run $(PKG_FLAGS)"

# Watch Level 2: Primary + Doc tests.
#
# Usage :
#	make wtest2 [crate=name]
wtest2:
	@echo "Watching Level 2: Primary + Doc tests..."
	@cargo watch -c -x "nextest run $(PKG_FLAGS)" -x "test --doc $(PKG_FLAGS)"

# Watch Level 3: Primary + Doc + Linter.
#
# Usage :
#	make wtest3 [crate=name]
wtest3:
	@echo "Watching Level 3: All standard checks..."
	@cargo watch -c -x "nextest run $(PKG_FLAGS)" -x "test --doc $(PKG_FLAGS)" -x "clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings"

# Watch Level 4: All standard + Heavy testing.
#
# Usage :
#	make wtest4 [crate=name]
wtest4:
	@echo "Watching Level 4: All checks + Heavy testing..."
	@cargo watch -c --shell "RUSTFLAGS=\"-D warnings\" cargo nextest run $(PKG_FLAGS) && RUSTDOCFLAGS=\"-D warnings\" cargo test --doc $(PKG_FLAGS) && cargo clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings && cargo +nightly udeps --all-targets $(PKG_FLAGS) && cargo +nightly audit $(PKG_FLAGS) && make --no-print-directory clean-cache-files"

# Watch Level 5: Full heavy testing with mutations.
#
# Usage :
#	make wtest5 [crate=name]
wtest5:
	@echo "Watching Level 5: Full heavy testing..."
	@cargo watch -c --shell "RUSTFLAGS=\"-D warnings\" cargo nextest run $(PKG_FLAGS) && RUSTDOCFLAGS=\"-D warnings\" cargo test --doc $(PKG_FLAGS) && cargo clippy --all-targets --all-features $(PKG_FLAGS) -- -D warnings && willbe .test dry:0 && cargo +nightly udeps --all-targets $(PKG_FLAGS) && cargo +nightly audit $(PKG_FLAGS) && make --no-print-directory clean-cache-files"

#
# === Cache Cleanup ===
#

# Clean cache files created by cargo audit and other tools by adding hyphen prefix.
# This ensures they are ignored by git while preserving the data for future runs.
#
# Usage :
#	make clean-cache-files
clean-cache-files:
	@echo "Cleaning cache files (adding hyphen prefix for git exclusion)..."
	@if [ -d "advisory-db" ]; then mv advisory-db -advisory-db 2>/dev/null || true; fi
	@if [ -f "advisory-db..lock" ]; then mv advisory-db..lock -advisory-db..lock 2>/dev/null || true; fi
	@if [ -d ".global-cache" ]; then mv .global-cache -.global-cache 2>/dev/null || true; fi
	@if [ -d ".package-cache" ]; then mv .package-cache -.package-cache 2>/dev/null || true; fi
	@if [ -d "registry" ]; then mv registry -registry 2>/dev/null || true; fi
	@echo "Cache files cleaned successfully."
