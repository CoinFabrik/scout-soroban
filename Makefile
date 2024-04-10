ci: fmt-rust lint test
ci-no-test: fmt-rust lint

fmt-rust:
	@echo "\nFormatting Rust code..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-fmt.py --dir $$dir; \
		fi; \
	done

lint:
	@echo "\nLinting detectors and test-cases..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-clippy.py --dir $$dir; \
		fi; \
	done

test:
	@echo "\nRunning tests..."
	@for detector in test-cases/*; do \
		if [ -d "$$detector" ]; then \
			python3 scripts/run-tests.py --detector=$$detector_name; \
		fi; \
	done

