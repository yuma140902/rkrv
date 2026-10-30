# rkrv

Rust Kernel for RISC-V

## 開発環境のセットアップ

x86_64 の Ubuntu 24.04 を前提にする。

```sh
sudo update
# 参考: [Install Ubuntu on QEMU (RISC-V) - Ubuntu hardware support documentation](https://ubuntu.com/hardware/docs/boards/how-to/ubuntu_supported/qemu-riscv/)
# U-Boot 無しで OpenSBI -> rkrv で起動するので u-boot-qemu は不要
sudo apt install qemu-system-riscv64 opensbi
# デバッグ用。ビルドには不要
sudo apt install binutils-riscv64-linux-gnu gdb-multiarch
mise i
```

## ビルド

```sh
cargo b
```

## QEMU で起動

```sh
./scripts/qemu.py
```

Ctrl+a, x で終了する。Ctrl+a, c で qemu monitor を切り替える。

## QEMU と gdb でデバッグ

```sh
./scripts/qemu.py -S
```

QEMU を起動し、CPU を一時停止させる。1234 番ポートで gdb を待ち受ける。

```sh
./scripts/gdb.py
```

gdb を起動し、QEMU に接続する。

↓のような感じで普通にデバッグできる。素晴らしい。

```
$ ./scripts/qemu.py -S
$ ./scripts/gdb.py
GNU gdb (Ubuntu 15.1-1ubuntu1~24.04.1) 15.1
Copyright (C) 2024 Free Software Foundation, Inc.
License GPLv3+: GNU GPL version 3 or later <http://gnu.org/licenses/gpl.html>
This is free software: you are free to change and redistribute it.
There is NO WARRANTY, to the extent permitted by law.
Type "show copying" and "show warranty" for details.
This GDB was configured as "x86_64-linux-gnu".
Type "show configuration" for configuration details.
For bug reporting instructions, please see:
<https://www.gnu.org/software/gdb/bugs/>.
Find the GDB manual and other documentation resources online at:
    <http://www.gnu.org/software/gdb/documentation/>.

For help, type "help".
Type "apropos word" to search for commands related to "word"...
Reading symbols from target/riscv64gc-unknown-none-elf/debug/rkrv...
The target architecture is set to "riscv:rv64".
Remote debugging using :1234
0x0000000000001000 in ?? ()
(gdb) b rust_main
Breakpoint 1 at 0x80200024: file src/main.rs, line 30.
(gdb) c
Continuing.

Breakpoint 1, rkrv::rust_main (_hart_id=0, _dtb_address=2279604224) at src/main.rs:30
30          loop {
(gdb) x/10i $pc
=> 0x80200024 <rkrv::rust_main+8>:      j       0x80200026 <rkrv::rust_main+10>
   0x80200026 <rkrv::rust_main+10>:     auipc   ra,0x0
   0x8020002a <rkrv::rust_main+14>:     jalr    -16(ra)
   0x8020002e <rkrv::rust_main+18>:     j       0x80200026 <rkrv::rust_main+10>
   0x80200030:  unimp
   0x80200032:  unimp
   0x80200034:  unimp
   0x80200036:  unimp
   0x80200038:  unimp
   0x8020003a:  unimp
(gdb) list
25      "#
26      );
27
28      #[unsafe(no_mangle)]
29      extern "C" fn rust_main(_hart_id: usize, _dtb_address: usize) -> ! {
30          loop {
31              unsafe { core::arch::riscv64::wfi() }
32          }
33      }
34
(gdb)
```
