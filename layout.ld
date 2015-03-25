INCLUDE ./exceptions.ld
INCLUDE ./interrupts.ld
INCLUDE ./registers.ld

MEMORY
{
  ROM(RX)   : ORIGIN = 0x08000000, LENGTH = 128K
  RAM(WAIL) : ORIGIN = 0x20000000, LENGTH = 8K
}

SECTIONS
{
  .text :
  {
      KEEP(*(.exception_vector))
      KEEP(*(.interrupt_vector))
      *(.text*)
      *(.rodata*)

      __TEXT_END__ = .;
  } > ROM

  .data : AT(__TEXT_END__)
  {
    __DATA_START__ = .;

		*(.data*)
    . = ALIGN(4);

    __DATA_END__ = .;
  } > RAM

  .bss :
  {
    __BSS_START__ = .;

		*(.bss*)
		*(COMMON)
    . = ALIGN(4);

    __BSS_END__ = .;
  } > RAM

  /DISCARD/ :
  {
    *(.ARM.exidx*)
  }

  __STACK_START__ = ORIGIN(RAM) + LENGTH(RAM);
}
