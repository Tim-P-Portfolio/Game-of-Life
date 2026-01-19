#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    Board,
    display::{self, blocking::Display, nonblocking::Display},
    gpio::DisplayPins,
    hal::timer,
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let timer0 = board.TIMER0;
    let display = Display::new(timer0, board.display_pins);

    loop {}
}
