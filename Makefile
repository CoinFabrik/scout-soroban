.PHONY: ci fmt-rust lint test ci-no-test

ci: fmt-rust lint test

fmt-rust:
	@echo "Formatting Rust code..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			echo "Formatting $$dir..."; \
			python3 scripts/run-fmt.py --dir $$dir; \
		fi; \
	done

lint:
	@echo "Linting detectors and test-cases..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			echo "Linting $$dir..."; \
			python3 scripts/run-clippy.py --dir $$dir; \
		fi; \
	done

test:
	@echo "Running tests..."
	@for detector in test-cases/*; do \
		if [ -d "$$detector" ]; then \
			detector_name=$$(basename $$detector); \
			python3 scripts/run-tests.py --detector=$$detector_name; \
		fi; \
	done

ci-no-test: fmt-rust lint
