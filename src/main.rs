#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    Board,
    display::{self, nonblocking::Display},
    gpio::DisplayPins,
    hal::{self, timer},
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

const FPS: u32 = 10u32;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer1 = hal::Timer::new(board.TIMER1);
    let display = Display::new(board.TIMER0, board.display_pins);

    loop {
        // 10 fps
        timer1.delay_ms(1000 / FPS);
    }
}
