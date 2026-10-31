OUTPUT_ARCH(riscv)
ENTRY(_start)

SECTIONS
{
    . = 0x80200000;

    .text : ALIGN(4K) {
        KEEP(*(.text.entry))
        *(.text .text.*)
    }

    .rodata : ALIGN(4K) {
        *(.rodata .rodata.*)
        . = ALIGN(4K);
    }

    .data : ALIGN(4K) {
        *(.data .data.*)
        . = ALIGN(4K);
    }

    .sdata : ALIGN(4K) {
        __sdata_start = .;
        __global_pointer$ = . + 0x800;
        *(.sdata .sdata.*)
        . = ALIGN(4K);
        __sdata_end = .;
    }

    .sbss : ALIGN(4K) {
      __sbss_start = .;
      *(.sbss .sbss.*)
        . = ALIGN(4K);
      __sbss_end = .;
    }

    .bss : ALIGN(4K) {
        __bss_start = .;
        *(.bss.stack)
        *(.bss .bss.*)
        *(COMMON)
        . = ALIGN(4K);
        __bss_end = .;
    }

    /DISCARD/ : {
        *(.eh_frame)
    }
}
