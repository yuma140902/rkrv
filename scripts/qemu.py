#!/usr/bin/env python3
import argparse
import pathlib
import subprocess


def main(args: argparse.Namespace):
    is_release: bool = args.release
    is_suspended: bool = args.suspended
    variant = "release" if is_release else "debug"

    subprocess.run(
        ["cargo", "build"] + (["--release"] if is_release else []),
        check=True,
        cwd=pathlib.Path(__file__).parent.parent,
    )

    a = ["qemu-system-riscv64"]
    a += ["-machine", "virt"]
    a += ["-cpu", "rv64"]
    a += ["-m", "128M"]
    a += ["-smp", "1"]
    a += ["-nographic"]
    a += ["-bios", "default"]
    a += ["-kernel", f"target/riscv64gc-unknown-none-elf/{variant}/rkrv"]
    if is_suspended:
        a += ["-S", "-gdb", "tcp::1234"]
    subprocess.run(
        a,
        check=True,
        cwd=pathlib.Path(__file__).parent.parent,
    )


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Run GDB")
    parser.add_argument(
        "-r", "--release", action="store_true", help="Use release build"
    )
    parser.add_argument(
        "-S", "--suspended", action="store_true", help="Start QEMU in suspended mode"
    )
    args = parser.parse_args()
    main(args)
