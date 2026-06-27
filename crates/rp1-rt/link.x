ENTRY(Reset);

MEMORY
{
  RP1_SRAM (rwx) : ORIGIN = 0x20000000, LENGTH = 64K
}

_stack_start = ORIGIN(RP1_SRAM) + LENGTH(RP1_SRAM);

SECTIONS
{
  .vector_table ORIGIN(RP1_SRAM) : ALIGN(4)
  {
    KEEP(*(.vector_table .vector_table.*));
  } > RP1_SRAM

  .text : ALIGN(4)
  {
    *(.text .text.*);
    *(.rodata .rodata.*);
  } > RP1_SRAM

  .data : ALIGN(4)
  {
    *(.data .data.*);
  } > RP1_SRAM

  .bss (NOLOAD) : ALIGN(4)
  {
    __sbss = .;
    *(.bss .bss.*);
    *(COMMON);
    __ebss = .;
  } > RP1_SRAM

  __image_end = .;

  /DISCARD/ :
  {
    *(.ARM.exidx*);
    *(.comment);
  }
}
