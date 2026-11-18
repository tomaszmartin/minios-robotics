use core::ptr::write_volatile;

// We define ports used by LED PINs
const P0: usize = 0x5000_0000;
const P1: usize = 0x5000_0300;

const OUTSET: usize = 0x508;
const OUTCLR: usize = 0x50C;
const DIRSET: usize = 0x518;

// This specifies processor PINs
const ROWS: [(usize, u32); 5] = [(P0, 21), (P0, 22), (P0, 15), (P0, 24), (P0, 19)];
const COLS: [(usize, u32); 5] = [(P0, 28), (P0, 11), (P0, 31), (P1, 5), (P0, 30)];

#[inline(never)]
fn reg_write(address: usize, value: u32) {
    unsafe {
        write_volatile(address as *mut u32, value);
    }
}

fn mask(pin: u32) -> u32 {
    let base: u32 = 1;
    base << pin
}

fn blank_rows() {
    for (port, pin) in ROWS {
        reg_write(port + OUTCLR, mask(pin));
    }
}

// select specific row and set specified pixels
fn drive_row(row: usize, pixels: &[u8; 5]) {
    // we deselect all rows
    blank_rows();

    // we reset all columns (HIGH turns them OFF)
    for (port, pin) in COLS {
        reg_write(port + OUTSET, mask(pin));
    }

    // we activate new columns
    for (col, &(port, pin)) in COLS.iter().enumerate() {
        if pixels[col] != 0 {
            reg_write(port + OUTCLR, mask(pin));
        }
    }

    // we activate current row
    let (port, pin) = ROWS[row];
    reg_write(port + OUTSET, mask(pin));
}

pub struct LedMatrix;

impl LedMatrix {
    pub fn new() -> Self {
        // we start with all rows deactivated
        blank_rows();

        for (port, pin) in COLS {
            reg_write(port + OUTSET, mask(pin));
        }

        for (port, pin) in ROWS {
            reg_write(port + DIRSET, mask(pin));
        }
        for (port, pin) in COLS {
            reg_write(port + DIRSET, mask(pin));
        }

        Self
    }

    pub fn show(&self, image: &[[u8; 5]; 5], cycles: u32) {
        for _ in 0..cycles {
            for (row, pixels) in image.iter().enumerate() {
                drive_row(row, pixels);
                cortex_m::asm::delay(100_000);
            }
        }
        blank_rows();
    }
}
