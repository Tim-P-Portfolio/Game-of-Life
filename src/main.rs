#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{
    Board,
    display::{self, nonblocking::Display},
    gpio::DisplayPins,
    hal::timer,
};
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let timer1 = board.TIMER1;
    let display = Display::new(board.TIMER0, board.display_pins);

    loop {}
}
