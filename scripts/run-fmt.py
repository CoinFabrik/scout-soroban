import argparse
import os
import time

from utils import print_errors, print_results, run_subprocess

RED = "\033[91m"
GREEN = "\033[92m"
BLUE = "\033[94m"
ENDC = "\033[0m"


def run_fmt(directories):
    errors = []
    for directory in directories:
        if not os.path.isdir(directory):
            errors.append(
                f"Error: The specified path '{directory}' is not a directory or does not exist."
            )
            continue

        print(f"\n{GREEN}Checking format in {directory}:{ENDC}")
        for root, _, files in os.walk(directory):
            if "Cargo.toml" in files:
                start_time = time.time()
                returncode, _, stderr = run_subprocess(
                    ["cargo", "fmt", "--all"],
                    cwd=root,
                )
                print_results(
                    returncode, stderr, "format", root, time.time() - start_time
                )
                if returncode != 0:
                    errors.append(root)
    return errors


if __name__ == "__main__":
    parser = argparse.ArgumentParser(
        description="Run cargo-fmt for specified directories"
    )
    parser.add_argument(
        "--dir",
        nargs="+",
        required=True,
        help="Specify the directories to run cargo-fmt on. Multiple directories can be specified.",
    )
    args = parser.parse_args()

    errors = run_fmt(args.dir)
    print_errors(errors)
    if errors:
        exit(1)
