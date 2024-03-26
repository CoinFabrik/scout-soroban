import os
import subprocess

RED = '\033[91m'
GREEN = '\033[92m'
ENDC = '\033[0m'

def run_fmt(directories):
    errors = []
    for directory in directories:
        print(f"\n{GREEN}Checking format in {directory}:{ENDC}")
        for root, _, files in os.walk(directory):
            if 'Cargo.toml' in files:
                print(f"Checking format in: {root}")
                result = subprocess.run(['cargo', '+nightly', 'fmt', '--', '--check', '-v'], cwd=root, capture_output=True, text=True)
                if result.returncode != 0:
                    print(f"\n{RED}Formatting issues found in: {root}{ENDC}\n")
                    error_message = result.stdout.strip()
                    for line in error_message.split('\n'):
                        print(f"| {line}")
                    print("\n")
                    errors.append(root)
    return errors

def print_fmt_errors(errors):
    if errors:
        print(f"{RED}\nFormatting errors detected in the following directories:{ENDC}")
        for error_dir in errors:
            print(f"• {error_dir}")
    else:
        print(f"{GREEN}\nNo formatting issues found across all directories.{ENDC}")

if __name__ == "__main__":
    directories = ['test-cases', 'detectors']
    errors = run_fmt(directories)
    print_fmt_errors(errors)
    if errors:
        exit(1)
