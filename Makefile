ci: fmt lint test
ci-no-test: fmt lint

fmt:
	@echo "\nFormatting test cases..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-fmt.py --dir $$dir; \
		fi; \
	done
	@echo "\nFormatting detectors..."
	@for dir in detectors/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-fmt.py --dir $$dir; \
		fi; \
	done


lint:
	@echo "\nLinting test cases..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-clippy.py --dir $$dir; \
		fi; \
	done
	@echo "\nLinting detectors..."
	@for dir in detectors/*; do \
		if [ -d "$$dir" ]; then \
			python3 scripts/run-clippy.py --dir $$dir; \
		fi; \
	done

test:
	@echo "\nRunning tests for test cases..."
	@for dir in test-cases/*; do \
		if [ -d "$$dir" ]; then \
			detector_name=$$(basename "$$dir"); \
			python3 scripts/run-tests.py --detector=$$detector_name; \
		fi; \
	done
