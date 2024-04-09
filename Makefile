ci: fmt-rust lint test
ci-no-test: fmt-rust lint

fmt-rust:
	@echo "Formatting Rust code..."
	@python3 scripts/run-fmt.py

lint:
	@echo "Linting detectors and test-cases..."
	@python3 scripts/run-clippy.py

test:
	@echo "Generating test matrix and running tests..."
	@for detector in test-cases/*; do \
		if [ -d "$$detector" ]; then \
			detector_name=$$(basename $$detector); \
			python3 scripts/run-tests.py --detector=$$detector_name; \
		fi \
	done
