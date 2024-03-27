import os
import subprocess
import argparse
import time

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def run_tests(detector):
    errors = []
    directory = os.path.join("test-cases", detector)
    print(f"\n{GREEN}Performing tests in {directory}:{ENDC}")
    if os.path.exists(directory):
        for root, _, files in os.walk(directory):
            if "Cargo.toml" in files:
                start_time = time.time()
                result = subprocess.run(
                    ["cargo", "test", "--all-features", "--all"],
                    cwd=root,
                    capture_output=True,
                    text=True,
                )
                end_time = time.time()
                elapsed_time = end_time - start_time
                print(
                    f"{BLUE}[> {elapsed_time:.2f} sec]{ENDC} - Completed test in: {root}."
                )
                if result.returncode != 0:
                    print(f"\n{RED}Test error found in: {root}{ENDC}\n")
                    error_message = result.stdout.strip()
                    for line in error_message.split("\n"):
                        print(f"| {line}")
                    print("\n")
                    errors.append(root)
    else:
        print(
            f"{RED}The specified detector directory does not exist: {directory}{ENDC}"
        )
    return errors


def print_tests_errors(errors):
    if errors:
        print(f"{RED}\nErrors detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo errors found in the specified directory.{ENDC}")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run tests for a specific detector.")
    parser.add_argument(
        "--detector",
        type=str,
        help='The detector to run tests for, e.g., "./unsafe-unwrap"',
    )

    args = parser.parse_args()

    if args.detector:
        errors = run_tests(args.detector.strip("./"))
        print_tests_errors(errors)
        if errors:
            exit(1)
    else:
        print(f"{RED}No detector specified. Please provide a detector argument.{ENDC}")
