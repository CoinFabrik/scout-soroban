import os
import argparse
import time

from utils import parse_json_from_string, print_errors, print_results, run_subprocess

RED = "\033[91m"
GREEN = "\033[92m"
ENDC = "\033[0m"


def run_tests(detector):
    errors = []
    directory = os.path.join("test-cases", detector)
    print(f"\n{GREEN}Performing tests in {directory}:{ENDC}")
    if not os.path.exists(directory):
        print(f"{RED}The specified directory does not exist.{ENDC}")
        return errors

    for root, _, files in os.walk(directory):
        if "Cargo.toml" in files:
            if run_unit_tests(root):
                errors.append(root)
            if run_integration_tests(detector, root):
                errors.append(root)
    return errors


def run_unit_tests(root):
    start_time = time.time()
    returncode, stdout, _ = run_subprocess(
        ["cargo", "test", "--all-features", "--all"], root
    )
    print_results(
        returncode,
        stdout,
        "unit-test",
        root,
        time.time() - start_time,
    )
    return returncode != 0


def run_integration_tests(detector, root):
    start_time = time.time()
    returncode, stdout, _ = run_subprocess(
        ["cargo", "+nightly-2023-12-16", "scout-audit", "--filter", detector, "--metadata"], root
    )

    if stdout is None:
        print(
            f"{RED}Failed to run integration tests in {root} - Metadata returned empty.{ENDC}"
        )
        return True

    detector_metadata = parse_json_from_string(stdout)

    if not isinstance(detector_metadata, dict):
        print("Failed to extract JSON:", detector_metadata)
        return True

    detector_key = detector.replace("-", "_")
    short_message = detector_metadata.get(detector_key, {}).get("short_message")

    returncode, _, stderr = run_subprocess(
        ["cargo", "+nightly-2023-12-16", "scout-audit", "--filter", detector], root
    )

    should_lint = root.endswith("vulnerable-example")
    if should_lint and short_message and short_message not in stderr:
        returncode = 1

    print_results(
        returncode,
        stderr,
        "integration-test",
        root,
        time.time() - start_time,
    )
    return returncode != 0


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run tests for a specific detector.")
    parser.add_argument(
        "--detector",
        type=str,
        required=True,
        help='The detector to run tests for, e.g., "unsafe-unwrap"',
    )
    args = parser.parse_args()

    errors = run_tests(args.detector)
    print_errors(errors)
    if errors:
        exit(1)
