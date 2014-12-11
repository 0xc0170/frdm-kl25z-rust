/*
 * KL25Z linker script file, 0xc0170
 */

MEMORY
{
  VECTORS (rx) : ORIGIN = 0x00000000, LENGTH = 0x00000400
  FLASH_PROTECTION	(rx) : ORIGIN = 0x00000400, LENGTH = 0x00000010
  FLASH (rx) : ORIGIN = 0x00000410, LENGTH = 128K - 0x00000410
  RAM (rwx) : ORIGIN = 0x1FFFF0C0, LENGTH = 16K - 0xC0
}

ENTRY(reset_handler)

SECTIONS
{
    .isr_vector :
    {
        __vector_table = .;
        KEEP(*(.vector_table))
         . = ALIGN(4);
    } > VECTORS

    .flash_protect :
    {
        KEEP(*(.flash_configuration))
         . = ALIGN(4);
    } > FLASH_PROTECTION

    .text :
    {
        __text = .;
        *(.text*)
        *(.rodata .rodata.*)
        __etext = .;
    } > FLASH

    .data : AT (__etext)
    {
        __data_start__ = .;
        *(.data*)
        . = ALIGN(4);
        __data_end__ = .;
    } > RAM

    .bss :
    {
        __bss_start__ = .;
        *(.bss*)
        *(COMMON)
        __bss_end__ = .;
    } > RAM

    .ARM.exidx : 
    {
        __exidx_start = .;
        *(.ARM.exidx* .gnu.linkonce.armexidx.*)
        __exidx_end = .;
    } > FLASH

    .heap :
    {
        __end__ = .;
        *(.heap*)
        __HeapLimit = .;
    } > RAM

    .stack_dummy :
    {
        *(.stack)
    } > RAM

    /* Set stack top to end of RAM, and stack limit move down by
     * size of stack_dummy section */
    __StackTop = ORIGIN(RAM) + LENGTH(RAM);
    __StackLimit = __StackTop - SIZEOF(.stack_dummy);
    PROVIDE(__stack = __StackTop);

    /* Check if data + heap + stack exceeds RAM limit */
    ASSERT(__StackLimit >= __HeapLimit, "region RAM overflowed with stack")
}
