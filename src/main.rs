#![no_main]
#![no_std]

mod life;
use life::*;

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use microbit::{Board, display::blocking::Display, hal::rng::Rng, hal::timer::Timer};
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

    rprintln!("");

    rprintln!("\r{}", MICROBIT[0]);
    rprintln!("\r{}", MICROBIT[1]);
    rprintln!("\r{}", MICROBIT[2]);
    rprintln!("\r{}", MICROBIT[3]);

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

    rprintln!("\r{}", MICROBIT[4]);
    rprintln!("\r{}", MICROBIT[5]);
    rprintln!("\r{}", MICROBIT[6]);

    loop {
        life(&mut grid);

        display.show(&mut timer1, grid, 1000 / FPS);
    }
}
