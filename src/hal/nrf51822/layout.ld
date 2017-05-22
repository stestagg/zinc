_data_load = LOADADDR(.data);

INCLUDE iomem.ld

ENTRY(main)

MEMORY
{
    rom(rx)   : ORIGIN = 0x00000000, LENGTH = 0x28000
    ram(wail) : ORIGIN = 0x20000000, LENGTH = 0x2000 - 4K
}

__STACK_BASE = ORIGIN(ram) + LENGTH(ram);

OUTPUT_FORMAT ("elf32-littlearm", "elf32-bigarm", "elf32-littlearm")

REGION_ALIAS("vectors", rom);

INCLUDE layout_common.ld
