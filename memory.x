/* memory.x - Linker script for the STM32F051C8T6 */
MEMORY
{
  /* Flash memory begins at 0x80000000 and has a size of 64kB*/
  FLASH : ORIGIN = 0x08000000, LENGTH = 64K
  /* RAM begins at 0x20000000 and has a size of 8kB*/
  RAM : ORIGIN = 0x20000000, LENGTH = 8K
}
