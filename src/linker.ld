OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x80200000; /* Here is where our kernel will be located */

SECTIONS {
    . = BASE_ADDRESS; 

    stext = .;
    .text : {
        *(.text.entry) /* rustsbi entry section */
        *(.text .text.*)
    }

    . = ALIGN(4K);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(4K);
    erodata = .;
    sdata = .;
    .data : {
        *(.daat .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(4k);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*) 
        *(.sbss .sbss.*)
    }

    . = ALIGN(4k);
    ebss = .;

    /*
        To find more about /DISCARD/:
        <https://sourceware.org/binutils/docs/ld/Output-Section-Discarding.html#Output-Section-Discarding>
    */
    /DISCARD/ : { /* To force-frame-pointers */
        *(.eh_frame)
    }
}
