OUTPUT_ARCH( "riscv" )
ENTRY( _start )

SECTIONS
{
    . = 0x80000000;
    skernel = .;

    . = ALIGN(0x1000);
    stext = .;
    .text : {
        *(.text.entry)
        *(.text .text.*)
    }

    . = ALIGN(0x1000);
    etext = .;
    srodata = .;
    .rodata : {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
    }

    . = ALIGN(0x1000);
    erodata = .;
    sdata = .;
    .data : {
        *(.data .data.*)
        *(.sdata .sdata.*)
    }

    . = ALIGN(0x1000);
    edata = .;
    .bss : {
        *(.bss.stack)
        sbss = .;
        *(.bss .bss.*)
        *(.sbss .sbss.*)
    }


    . = ALIGN(0x1000);
    ebss = .;
    ekernel = .;
}