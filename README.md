# rvrk

RISC-V Rust Kernel

## 開発環境のセットアップ

```sh
sudo update
# 参考: [Install Ubuntu on QEMU (RISC-V) - Ubuntu hardware support documentation](https://ubuntu.com/hardware/docs/boards/how-to/ubuntu_supported/qemu-riscv/)
# U-Boot 無しで OpenSBI -> rvrk で起動するので u-boot-qemu は不要
sudo apt install qemu-system-riscv64 opensbi
sudo apt install binutils-riscv64-linux-gnu gdb-multiarch
mise i
```
