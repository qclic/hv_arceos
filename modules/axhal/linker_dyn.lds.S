OUTPUT_ARCH(Aarch64)

ENTRY(_start)


INCLUDE pie_boot.x

SECTIONS
{
    text : ALIGN(4K) {
        *any_uart-*.o*(.text*)
        *fdt-parser-*.o*(.text*)
        *(.text .text.*)
    } 

    .rodata : ALIGN(4K) {
        __srodata = .;
        *(.rodata .rodata.*)
        *(.srodata*)
        *(.got .got.*)
    } 
    __custom_rodata = .;
    
    .data : ALIGN(4K) {
        __erodata = .;
        __srwdata = .;
        *(.data*)
        . = ALIGN(4K);
        __global_pointer$ = . + 0x800;
        *(.sdata.*)
    } 
    . = ALIGN(4K);
    __custom_rwdata = .;

    percpu : ALIGN(4K) {
        __kernel_load_size = . - __kernel_load_vma;
        __erwdata = .;

        *(.percpu*)
    } 

    bss (NOLOAD) : ALIGN(4K)  {
        *(.bss .bss.*)
        *(.sbss .sbss.*)
        *(COMMON)
        . = ALIGN(4K);

        __kernel_code_end = .;
    } 


	/DISCARD/ : {
        *(.comment) *(.gnu*) *(.note*) *(.eh_frame*)
    }
} 

SECTIONS {
    .init_array : ALIGN(0x10) {
         __init_array_start = .;
         *(.init_array .init_array.*)
         __init_array_end = .;
     }

     linkme_IRQ : { *(linkme_IRQ) }
     linkm2_IRQ : { *(linkm2_IRQ) }
     linkme_PAGE_FAULT : { *(linkme_PAGE_FAULT) }
     linkm2_PAGE_FAULT : { *(linkm2_PAGE_FAULT) }
     linkme_SYSCALL : { *(linkme_SYSCALL) }
     linkm2_SYSCALL : { *(linkm2_SYSCALL) }
     axns_resource : { *(axns_resource) }
 }
 INSERT AFTER .data