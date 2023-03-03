  .section .text.entry
  .globl _start
_start:
  la sp, boot_stack_top
  call rust_main

  .section .bss.stack
  .globl boot_stack
boot_stack:
  .space 64 * 1024 // 64 KiB
  .globl boot_stack_top
boot_stack_top: