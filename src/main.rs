#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

// Separate function: easier to breakpoint in GDB
#[inline(never)]
fn next_tick(value: u32) -> u32 {
    value.wrapping_add(1)
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("MiniOS Robotics: kernel start");

    let mut tick: u32 = 0;

    loop {
        tick = next_tick(tick);
        rprintln!("tick = {}", tick);
        cortex_m::asm::delay(16_000_000);
    }
}
