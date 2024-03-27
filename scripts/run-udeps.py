import os
import subprocess

RED = '\033[91m'
GREEN = '\033[92m'
ENDC = '\033[0m'

def run_udeps(directories):
    errors = []
    for directory in directories:
        print(f"\n{GREEN}Checking unused dependencies in {directory}:{ENDC}")
        for root, _, files in os.walk(directory):
            if 'Cargo.toml' in files:
                print(f"Checking unused dependencies in: {root}")
                result = subprocess.run(['cargo', '+nightly', 'udeps', '--no-default-features'], cwd=root, capture_output=True, text=True)
                if result.returncode != 0:
                    print(f"\n{RED}Unused dependencies found in: {root}{ENDC}\n")
                    error_message = result.stdout.strip()
                    for line in error_message.split('\n'):
                        print(f"| {line}")
                    print("\n")
                    errors.append(root)
    return errors

def print_udeps_errors(errors):
    if errors:
        print(f"{RED}\nUnused dependencies detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo unused dependencies found across all directories.{ENDC}")

if __name__ == "__main__":
    directories = ['test-cases', 'detectors']
    errors = run_udeps(directories)
    print_udeps_errors(errors)
    if errors:
        exit(1)
