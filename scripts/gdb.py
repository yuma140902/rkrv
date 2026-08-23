#!/usr/bin/env python3
import argparse
import os
import pathlib
import subprocess


def main(args: argparse.Namespace):
    env = os.environ.copy()
    env["RUST_GDB"] = "gdb-multiarch"
    is_release: bool = args.release
    variant = "release" if is_release else "debug"
    subprocess.run(
        [
            "rust-gdb",
            f"target/riscv64gc-unknown-none-elf/{variant}/rkrv",
            "-ex",
            "set architecture riscv:rv64",
            "-ex",
            "target remote :1234",
        ],
        env=env,
        check=True,
        cwd=pathlib.Path(__file__).parent.parent,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run GDB")
    parser.add_argument(
        "-r", "--release", action="store_true", help="Use release build"
    )
    args = parser.parse_args()
    main(args)
