#!/usr/bin/env python3

import sys
import os
import subprocess

script_dir = os.path.dirname(os.path.realpath(__file__))
src_dir = os.path.join(script_dir, "..")
document_cli_usage_file = os.path.join(src_dir, sys.argv[1])


def run_cmd_capture_stdout(command, cwd, error_message):
    print(f"running command: {command}", file=sys.stderr)
    res = subprocess.run(
        command, cwd=cwd, capture_output=True, text=True, shell=True)
    if res.returncode != 0:
        print(f"{error_message}, code: {res.returncode}", file=sys.stderr)
        print(f"command: {command}", file=sys.stderr)
        sys.exit(1)
    return res.stdout


def cli_cmd(args):
    return "cargo run --manifest-path {} --target-dir {} -- {}".format(
        os.path.join(src_dir, "Cargo.toml"), os.path.join(src_dir, "target"), " ".join(args))


print(f"""
### executing document-cli-usage.py with args:
    document_cli_usage_file: {document_cli_usage_file}
""", file=sys.stderr)

# Opening a file in write mode clears its content
with open(os.path.join(src_dir, document_cli_usage_file), 'w') as file:
    # top-level help
    file.write("### top-level help: ### \n\n")
    file.write(run_cmd_capture_stdout(
        cli_cmd(["--help"]), src_dir, "running cli top-level help failed"))
    file.write("\n\n")

    # subcommand "keyboard" help
    file.write("### subcommand keyboard help: ###\n\n")
    file.write(run_cmd_capture_stdout(
        cli_cmd(["keyboard", "--help"]), src_dir, "running cli keyboard help failed"))
    file.write("\n\n")

    # subcommand "screen" help
    file.write("### subcommand screen help: ###\n\n")
    file.write(run_cmd_capture_stdout(
        cli_cmd(["screen", "--help"]), src_dir, "running cli screen help failed"))
    file.write("\n")
