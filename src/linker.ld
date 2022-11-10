OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80_200_000;

SECTIONS {
    . = BASE_ADDRESS; /* `.` represents the current phycial address */
    kernel_entry = .;

    text_entry = .; /* the start of text section */
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(4K);
    text_end = .; /* end text section */
    rodata_entry = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.rodata_entry .rodata_entry.*)
    }

    . = ALIGN(4K);
    rodata_end = .;
    data_entry = .;
    .data : {
        *(.daat .data.*)
        *(.data_entry .data_entry.*)
    }

    . = ALIGN(4k);
    data_end = .;
    .bss : {
        *(.bss.stack)
        bss_entry = .;
        *(.bss .bss.*)
        *(.bss_entry .bss_entry.*)
    }

    . = ALIGN(4k);
    bss_end = .;
    kernel_end = .;

    /*
        To find more about /DISCARD/:
        <https://sourceware.org/binutils/docs/ld/Output-Section-Discarding.html#Output-Section-Discarding>
    */
    /DISCARD/ : { /* To force-frame-pointers */
        *(.eh_frame)
    }
}
