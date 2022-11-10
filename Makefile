.DEFAULT_GOAL:=slime_kernel

KERNEL_ELF:=target/riscv64gc-unknown-none-elf/release/slime_os
KERNEL_BIN:=target/riscv64gc-unknown-none-elf/release/slime_kernel.bin

BOOTLOADER:=bootloader/rustsbi-qemu.bin # rustsbi-qemu
KERNEL_ENTRY_PA:=0x80200000 # rustsbi-qemu fn `entry` PA: phycial address

.PHONY: slime_kernel
slime_kernel:
	@cargo build --release
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

.PHONY: run
run: slime_kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-bios $(BOOTLOADER)
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PA)

.PHONY: clean
clean:
	@cargo clean
