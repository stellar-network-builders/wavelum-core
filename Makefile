# Code quality targets

fmt:
	cargo fmt --all
	cargo fmt --check --all
	@echo "Format complete."

lint:
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "Lint complete."

fmt-check:
	cargo fmt --check --all
	@echo "Format check complete."

lint-check:
	cargo clippy --workspace --all-targets -- -D warnings
	@echo "Lint check complete."

check: fmt-check lint-check
	@echo "All checks complete."

# Coverage target using cargo-llvm-cov
coverage:
	cargo install cargo-llvm-cov 2>/dev/null || true
	cargo llvm-cov --workspace --lcov --output-path lcov.info
	cargo llvm-cov --workspace --summary-only
	@echo "Coverage report: lcov.info"

coverage-html:
	cargo llvm-cov --workspace --html --output-dir coverage/
	@echo "HTML report: coverage/index.html"

coverage-clean:
	rm -f lcov.info
	rm -rf coverage/
