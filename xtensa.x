/* 

ENTRY(Reset);
MEMORY
{
  /* NOTE K = KiBi = 1024 bytes */
  /* TODO Adjust these memory regions to match your device memory layout */
  FLASH : ORIGIN = 0x40080000, LENGTH = 256K 
  /* RAM : ORIGIN = 0x20000000, LENGTH = 64K /*
}

/* # Sections */
SECTIONS
{
  PROVIDE(_stack_start = ORIGIN(RAM) + LENGTH(RAM));

  /* ## Sections in FLASH */
  /* ### Vector table */
  .iram0 ORIGIN(FLASH) :
  {
    /* Initial Stack Pointer (SP) value */
    LONG(_stack_start);

    /* Reset vector */
    KEEP(*(.iram0.vectors)); /* this is the `__RESET_VECTOR` symbol */
    __reset_vector = .;

  } > FLASH


}

*/