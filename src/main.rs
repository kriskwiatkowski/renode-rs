#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;

const UART0_BASE: usize = 0x4930_3000;

// CMSDK APB UART register offsets (in bytes)
const DATA:    usize = 0x00;
const STATE:   usize = 0x04;
const CTRL:    usize = 0x08;

const CTRL_TX_EN: u32 = 1 << 0;  // enable TX
const STATE_TX_FULL: u32 = 1 << 0; // TX buffer full

fn reg(offset: usize) -> *mut u32 {
    (UART0_BASE + offset) as *mut u32
}

fn uart_init() {
    // Enable TX — this is what was missing / wrong before
    unsafe { reg(CTRL).write_volatile(CTRL_TX_EN) };
}

fn uart_putc(c: u8) {
    // Wait while TX buffer is full
    unsafe {
        while reg(STATE).read_volatile() & STATE_TX_FULL != 0 {}
        reg(DATA).write_volatile(c as u32);
    }
}

fn uart_puts(s: &str) {
    for b in s.bytes() {
        uart_putc(b);
    }
}

#[entry]
fn main() -> ! {
    uart_init();
    uart_puts("Hello World from Cortex-M4 (Rust)!\n");
    loop {
        cortex_m::asm::wfe();
    }
}

