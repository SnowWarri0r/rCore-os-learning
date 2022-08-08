OUTPUT_ARCH(riscv)
ENTRY(_start)
BASE_ADDRESS = 0x10000;

SECTIONS
{
  . = BASE_ADDRESS;
  .text : {
    *(.text.entry)
    *(.text .text.*)
  }
  . = ALIGN(4K);
  .rodata : {
    *(.rodata .rodata.*)
    *(.srodata .srodata.*)
  }
  . = ALIGN(4K);
  # .data 和 .bss 两个逻辑段由于访问限制相同（可读写），它们中间则无需进行页面对齐。
  .data : {
    *(.data .data.*)
    *(.sdata .sdata.*)
  }
  start_bss = .;
  .bss : {
    *(.bss .bss.*)
    *(.sbss .sbss.*)
  }
  end_bss = .;
  /DISCARD/ : {
    *(.eh_frame)
    *(.debug*)
  }
}