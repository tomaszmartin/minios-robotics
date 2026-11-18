#![no_std]
#![no_main]

mod matrix;

use cortex_m as _;
use cortex_m_rt::entry;
use matrix::LedMatrix;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const HEART: [[u8; 5]; 5] = [
    [0, 1, 0, 1, 0],
    [1, 1, 1, 1, 1],
    [1, 1, 1, 1, 1],
    [0, 1, 1, 1, 0],
    [0, 0, 1, 0, 0],
];
const EMPTY: [[u8; 5]; 5] = [[0; 5]; 5];

// for testing purposes
fn show_heart() {
    let display = LedMatrix::new();
    display.show(&HEART, 200);
    display.show(&EMPTY, 100);
}

#[entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("MiniOS Robotics: kernel start");

    loop {
        show_heart()
    }
}
