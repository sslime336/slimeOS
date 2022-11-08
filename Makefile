.PHONY build debug run

build:
    cargo build --release
    rust-objcopy --strip-all target/riscv64gc-unknown-none-elf/release/os \
        -O binary target/riscv64gc-unknown-none-elf/release/os.bin

debug: build
    riscv64-unknown-elf-gdb \
        -ex 'file target/riscv64gc-unknown-none-elf/release/os' \
        -ex 'set arch riscv:rv64' \
        -ex 'target remote localhost:1234'

run: build
    qemu-system-riscv64 \
        -machine virt \
        -nographic \
        -bios ../bootloader/rustsbi-qemu.bin \
        -device loader,file=target/riscv64gc-unknown-none-elf/release/os.bin,addr=0x80200000 \
