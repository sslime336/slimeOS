.DEFAULT_GOAL = $(KERNEL)

KERNEL = slime_os

NHART = 5 	   # The number of harts
CPU = rv64     # riscv x64
MEM = 128M 	   # available memory, from 0x80000000 to 0x88000000

MACHINE = virt

KERNEL_PATH = kernel/target/riscv64gc-unknown-none-elf/release

KERNEL_ELF = $(KERNEL_PATH)/$(KERNEL)
KERNEL_BIN = $(KERNEL_PATH)/$(KERNEL).bin

BOOTLOADER = none

LOG_LEVEL = DEBUG


CRATE_KERNEL = kernel
CRATE_USER = user

${KERNEL}:
	@cd $(CRATE_KERNEL)
	@LOG_LV=${LOG_LEVEL} cargo build --release
	@rust-objcopy --strip-all $(KERNEL_ELF) -O binary $(KERNEL_BIN)

.PHONY: run
run: $(KERNEL)
	@$(QEMU) \
		-M $(MACHINE) \
		-serial mon:stdio \
		-nographic \
		-cpu $(CPU) \
		-bios $(BOOTLOADER) \
		-m $(MEM) \
		-kernel $(KERNEL_BIN) \
		-smp $(NHART)

.PHONY: debug-server
debug-server: $(KERNEL)
	@$(QEMU) \
		-M $(MACHINE) \
		-serial mon:stdio \
		-nographic \
		-cpu $(CPU) \
		-bios $(BOOTLOADER) \
		-m $(MEM) \
		-kernel $(KERNEL_BIN) \
		-smp $(NHART) \
		-s -S

.PHONY: debug
debug:
	@${GDB} \
		-ex 'file $(KERNEL_ELF)' \
		-ex 'set arch riscv:rv64' \
		-ex 'target remote localhost:1234'

.PHONY: clean-kernel
clean-kernel:
	@cd $(CRATE_KERNEL)
	@cargo clean

.PHONY: clean-user
clean-user:
	@cd $(CRATE_USER)
	@cargo clean

.PHONY: clean-all
clean-all: clean-kernel clean-user
	@echo [make] all previous build has been cleaned up.

################### Unused ###################
QEMU = qemu-system-riscv64
GDB = riscv64-unknown-elf-gdb

TOOLPREFIX = riscv64-unknown-elf-

CC = ${TOOLPREFIX}gcc
AS = ${TOOLPREFIX}as
LD = ${TOOLPREFIX}ld

CFLAGS = -march=rv64gc
##############################################
