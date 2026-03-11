MEMORY {
    /* MPS3 AN547 Code/SRAM regions */
    FLASH (rx)  : ORIGIN = 0x01000000, LENGTH = 2M
    RAM   (rwx) : ORIGIN = 0x20000000, LENGTH = 256K
}
