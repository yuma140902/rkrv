# Repository Guidelines

## Project Structure & Module Organization

`rkrv` is a Rust 2024, `no_std` RISC-V supervisor-mode kernel targeting QEMU's `virt` machine. `src/main.rs` contains boot assembly, kernel initialization, and demonstration tasks. `src/lib.rs` exposes kernel modules: `alloc.rs` for allocation, `sched.rs` for scheduling and context switching, `trap.rs` for traps and interrupt control, `per_hart.rs` for hart-local state, and `sbi.rs` for SBI calls.

`linker.ld` defines the memory layout; `.cargo/config.toml` selects `riscv64gc-unknown-none-elf` and the linker script. `scripts/` contains Python launchers. Build artifacts go in `target/`. There is no separate test or asset directory.

## Build, Test, and Development Commands

Use the toolchain pinned in `mise.toml` (`nightly-2026-08-16`). The README describes Ubuntu dependencies, including QEMU, OpenSBI, and optional debugging tools.

- `mise install`: install the configured development tools.
- `cargo build`: build the debug kernel for the configured RISC-V target.
- `cargo build --release`: build an optimized kernel with debug symbols.
- `./scripts/qemu.py`: build and boot the debug kernel; add `--release` for release mode. Exit with Ctrl+a, x.
- `./scripts/qemu.py -S`: boot paused with GDB listening on port 1234.
- `./scripts/gdb.py`: connect `gdb-multiarch` through `rust-gdb`; use `--release` to match a release kernel.
- `cargo fmt --check` and `cargo clippy`: check formatting and lints when their toolchain components are installed.

## Coding Style & Naming Conventions

Use four-space indentation and standard Rust formatting. Use `snake_case` for modules/functions, `PascalCase` for types, and `SCREAMING_SNAKE_CASE` for constants. Preserve `no_std` compatibility. Both crate roots deny `rust_2018_idioms`, `clippy::all`, and `clippy::nursery`. Document unsafe APIs with safety requirements and explain hardware assumptions near unsafe operations.

## Testing Guidelines

There is no configured automated test runner or coverage threshold. `src/sched.rs` contains a `#[cfg(test)]` scaffold without `#[test]` cases, and binary tests are disabled. Validate changes by building and booting in QEMU; check UART output and task switching. Use GDB for register, trap, and context-layout checks. Record validation steps and observed behavior in pull requests.

## Commit & Pull Request Guidelines

History uses short imperative subjects, sometimes prefixed with `feat:`, `refactor:`, or `doc:`; English and Japanese both appear. Follow that concise style. Pull requests should describe the affected kernel behavior, link relevant issues, and include build results and QEMU observations. Explain changes to memory layout, assembly, or interrupt invariants explicitly.
