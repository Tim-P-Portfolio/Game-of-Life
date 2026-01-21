#![no_main]
#![no_std]

mod life;
use life::*;

use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;

use cortex_m_rt::entry;
use microbit::{
    Board,
    display::blocking::Display,
    hal::{rng::Rng, timer::Timer},
};
use panic_halt as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

const MICROBIT: [&str; 7] = [
    // Top
    "╭───────────────────────╮",
    "│          ▁▁▁          │",
    "│         (● ●)         │",
    "│          ▔▔▔          │",
    // Bottom
    "│                       │",
    "│ ◯    ◯    ◯    ◯    ◯ │",
    r#"╰/▔\__/▔\__/▔\__/▔\__/▔\╯"#,
];

const FPS: u32 = 10;

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer1 = Timer::new(board.TIMER1);

    let mut display = Display::new(board.display_pins);
    let mut rng = Rng::new(board.RNG);

    let mut grid = [[0u8; 5]; 5];
    let mut row;

    let mut button_a = board.buttons.button_a.into_pullup_input();

    rprintln!("");

    for i in 0..4 {
        rprintln!("\r{}", MICROBIT[i]);
    }

    for c in 0..5 {
        row = [0; 5];
        rprint!("\r│      ");
        for r in 0..5 {
            // 0-127: 0, 128-255: 1
            let num = if rng.random_u8() > 127 { 1 } else { 0 };
            grid[c][r] = num;
            row[r] = num;
            rprint!("{}", if num == 1 { " ▮" } else { " ▯" });
        }
        rprint!("       │");

        rprintln!("");
    }

    for i in 5..7 {
        rprintln!("\r{}", MICROBIT[i]);
    }

    loop {
        life(&mut grid);

        let button_state = button_a.is_low().unwrap();
        if button_state == true {
            rprintln!("Low");
            timer1.delay_ms(1000 / FPS);
        } else {
        }

        display.show(&mut timer1, grid, 1000 / FPS);
    }
}
