ENTRY(_start)

SECTIONS {
    . = 0x100000;

    .text : {
        KEEP(*(.text.entry))
        *(.text .text.*)
    }

    .rodata : {
        *(.rodata .rodata.*)
    }

    .data : {
        *(.data .data.*)
    }

    .bss : {
        *(.bss .bss.*)
        *(COMMON)
    }

    .stack ALIGN(16) : {
        _stack_end = .;
        . = . + 0x10000;
        _stack_start = .;
    }
}