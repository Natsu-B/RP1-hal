ENTRY(Reset);

MEMORY
{
  FLASH (rx)  : ORIGIN = 0x20000000, LENGTH = 256K
  RAM   (rwx) : ORIGIN = 0x20040000, LENGTH = 256K
}

_stack_start = ORIGIN(RAM) + LENGTH(RAM);

SECTIONS
{
  .vector_table ORIGIN(FLASH) : ALIGN(4)
  {
    KEEP(*(.vector_table));
  } > FLASH

  .text : ALIGN(4)
  {
    *(.text .text.*);
    *(.rodata .rodata.*);
  } > FLASH

  .data : ALIGN(4)
  {
    *(.data .data.*);
  } > RAM AT> FLASH

  .bss (NOLOAD) : ALIGN(4)
  {
    *(.bss .bss.*);
    *(COMMON);
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*);
    *(.comment);
  }
}
