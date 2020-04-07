/* Specify main memory areas */
MEMORY
{
  /* Use values from the ESP-IDF 'bootloader' component. */
  vectors ( RX )       : ORIGIN = 0x40080000, len = 0x400
  iram_seg ( RX )       : ORIGIN = 0x40080400, len = 0xFC00
  dram_seg ( RW )       : ORIGIN = 0x3FFF0000, len = 0x1000
}