#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{Board, display::blocking::Display, hal::timer::Timer};
use panic_halt as _;
// use rtt_target::{rprintln, rtt_init_print};

const FPS: u32 = 10u32;

#[entry]
fn main() -> ! {
    // rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer1 = Timer::new(board.TIMER1);
    let mut display = Display::new(board.display_pins);

    let mut grid = [[0u8; 5]; 5];
    grid[0][1] = 1;

    loop {
        display.show(&mut timer1, grid, 100);
        // 10 fps
        timer1.delay_ms(1000 / (FPS / 2));
        display.clear();
        timer1.delay_ms(1000 / (FPS / 2));
    }
}
