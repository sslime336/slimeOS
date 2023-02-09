.DEFAULT_GOAL:=slime_kernel

KERNEL_ELF:=target/riscv64gc-unknown-none-elf/release/slime_os
KERNEL_BIN:=target/riscv64gc-unknown-none-elf/release/slime_kernel.bin

KERNEL_ENTRY_PHYSICAL_ADDRESS:=0x80200000 # our kernel's entry, actually the `rust_main` in main.rs

.PHONY: slime_kernel
slime_kernel:
	@cargo build --release
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

.PHONY: run
run: slime_kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PHYSICAL_ADDRESS)

.PHONY: debug-server
debug-server: slime_kernel
	@qemu-system-riscv64 \
		-machine virt \
		-nographic \
		-device loader,file=$(KERNEL_BIN),addr=$(KERNEL_ENTRY_PHYSICAL_ADDRESS) \
		-s -S 

.PHONY: debug
debug: 
	@riscv64-unknown-elf-gdb \
    -ex 'file $(KERNEL_ELF)' \
    -ex 'set arch riscv:rv64' \
    -ex 'target remote localhost:1234'

.PHONY: clean
clean:
	@cargo clean
