.DEFAULT_GOAL:=bin
TARGET:=target/riscv64gc-unknown-none-elf/release/slime_os
OUTPUT:=target/riscv64gc-unknown-none-elf/release/slime_kernel.bin

slime_kernel:

bin: build
	rust-objcopy --strip-all $(TARGET)  -O binary $(OUTPUT)
.PHONY: bin

build:

.PHONY: build

run: slime_kernel
	qemu-system-riscv64gc \
		-machine virt \
		-nographic \
		-bios TODO: dl rustsbi-qemu.bin
		-device loader,file=
.PHONY: run
