#![no_main]
#![no_std]

mod life;
use life::*;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{Board, display::blocking::Display, hal::rng::Rng, hal::timer::Timer};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const FPS: u32 = 10u32;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer1 = Timer::new(board.TIMER1);
    let mut display = Display::new(board.display_pins);
    let mut rng = Rng::new(board.RNG);
    /**
     * RNG
     * rng_value: read rng, u8
     * rng_start: trigger generation, bool
     * rng_stop: stop generation, bool
     * rng_valrdy: generated when each rng is created, bool
     */
    // let rng = board.RNG;
    // let rng_value = &rng.value;
    // let rng_start = &rng.tasks_start;
    // let rng_stop = &rng.tasks_stop;
    // let rng_valrdy = &rng.events_valrdy;

    // // rng::tasks_start::TASKS_START
    // let rng_start_val = rng::tasks_start::W::bits(&mut self, 1);
    // rng_start.write();

    rprintln!("{}", rng.random_u8());

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
