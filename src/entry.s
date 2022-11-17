  .section .text.entry // the first code
  .globl _start
_start:
  la sp, boot_stack_top // actually this line :P
  call rust_main

  .section .bss.stack
  .globl boot_stack
boot_stack:
  .space 64 * 1024 // 64 KiB
  .globl boot_stack_top
boot_stack_top:

/*
  Kernel Memory View
  |----------------|
  |     stack      |
  |----------------|
  |                |
  |                |  ---> unused memeoy
  |                |
  |                |
  |----------------|
  |      heap      |
  |----------------|
  |      .bss      |
  |----------------|
  |     .data      |
  |----------------|
  |    .rodata     |  ---> read only data section
  |----------------|
  |                |
  |     .text      |  ---> here is where our code located
  |                |
  |----------------|
  |                |
*/
