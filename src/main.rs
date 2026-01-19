#![no_main]
#![no_std]

mod life;
use life::*;

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

    /**
     * RNG
     * rng_value: read rng
     * rng_start: trigger generation
     * rng_stop: stop generation
     * rng_valrdy: event triggered when each rng is created
     */
    let rng = board.RNG;
    let rng_value = &rng.value;
    let rng_start = &rng.tasks_start;
    let rng_stop = &rng.tasks_stop;
    let rng_valrdy = &rng.events_valrdy;

    let mut grid = [[0u8; 5]; 5];
    for r in 0..5 {
        grid[r][1] = 1
    }

    // done, life

    loop {
        life(&mut grid);

        display.show(&mut timer1, grid, 100);
        // 10 fps
        timer1.delay_ms(1000 / (FPS / 2));
        display.clear();
        timer1.delay_ms(1000 / (FPS / 2));
    }
}
