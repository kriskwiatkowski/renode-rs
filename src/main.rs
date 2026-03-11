#![no_std]
#![no_main]

#[cfg(feature = "cycle-count")]
use cortex_m::peripheral::DWT;
use cortex_m_rt::entry;
use panic_halt as _;
use sha3::{Digest, Sha3_256};

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

#[cfg(feature = "cycle-count")]
fn uart_put_u32(mut n: u32) {
    if n == 0 {
        uart_putc(b'0');
        return;
    }
    let mut buf = [0u8; 10];
    let mut i = 10usize;
    while n > 0 {
        i -= 1;
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
    }
    for &c in &buf[i..] {
        uart_putc(c);
    }
}

fn uart_put_hex(bytes: &[u8]) {
    const HEX: &[u8] = b"0123456789abcdef";
    for &b in bytes {
        uart_putc(HEX[(b >> 4) as usize]);
        uart_putc(HEX[(b & 0xf) as usize]);
    }
}

#[entry]
fn main() -> ! {
    #[cfg(feature = "cycle-count")]
    let mut cp = cortex_m::Peripherals::take().unwrap();
    #[cfg(feature = "cycle-count")]
    cp.DCB.enable_trace();
    #[cfg(feature = "cycle-count")]
    cp.DWT.enable_cycle_counter();

    uart_init();
    uart_puts("Hello World from Cortex-M4 (Rust)!\n");

    #[cfg(feature = "cycle-count")]
    let start = DWT::cycle_count();
    let digest = Sha3_256::digest(b"The quick brown fox jumps over the lazy dog");
    #[cfg(feature = "cycle-count")]
    let cycles = DWT::cycle_count().wrapping_sub(start);

    uart_puts("SHA3-256: ");
    uart_put_hex(&digest);
    uart_putc(b'\n');
    #[cfg(feature = "cycle-count")]
    {
        uart_puts("Cycles: ");
        uart_put_u32(cycles);
        uart_putc(b'\n');
    }

    loop {
        cortex_m::asm::wfe();
    }
}

