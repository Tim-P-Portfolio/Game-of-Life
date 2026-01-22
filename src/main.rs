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

struct Grid {
    grid: [[u8; 5]; 5],
}

impl Grid {
    fn new() -> Self {
        Self { grid: [[0; 5]; 5] }
    }

    fn complement(&mut self) {
        for r in 0..5 {
            for c in 0..5 {
                self.grid[r][c] = 1 - self.grid[r][c]
            }
        }
    }

    fn generate_grid(&mut self, rng: &mut Rng) {
        // Display first 3 lines of the microbit in the terminal
        for i in 0..4 {
            rprintln!("\r{}", MICROBIT[i]);
        }

        for c in 0..5 {
            rprint!("\r│      ");
            for r in 0..5 {
                // 0-127: 0, 128-255: 1
                let num = if rng.random_u8() > 127 { 1 } else { 0 };
                self.grid[c][r] = num;
                rprint!("{}", if num == 1 { " ▮" } else { " ▯" });
            }
            rprint!("       │");

            rprintln!("");
        }

        // Display last 3 lines of the microbit in the terminal
        for i in 5..7 {
            rprintln!("\r{}", MICROBIT[i]);
        }
    }
}

enum GameState {
    ButtonAPressed,
    ButtonBPressed,
    Randomize,
    Running,
    Complement,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer1 = Timer::new(board.TIMER1);

    let mut display = Display::new(board.display_pins);
    let mut rng = Rng::new(board.RNG);

    let mut grid = Grid::new();

    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    rprintln!("");
    grid.generate_grid(&mut rng);

    let mut state = GameState::Randomize;

    loop {
        // Get buttonGameState
        let button_a_pressed = button_a.is_low().unwrap();
        let button_b_pressed = button_b.is_low().unwrap();

        state = match state {
            GameState::ButtonAPressed => {
                rprintln!("A low");
                GameState::Randomize
            }
            GameState::ButtonBPressed => {
                rprintln!("Low");
                GameState::Complement
            }
            GameState::Randomize => {
                grid.generate_grid(&mut rng);
                GameState::Running
            }
            GameState::Complement => {
                grid.complement();
                GameState::Running
            }
            GameState::Running => {
                life(&mut grid.grid);

                if button_a_pressed {
                    GameState::ButtonAPressed
                } else if button_b_pressed {
                    GameState::ButtonBPressed
                } else {
                    GameState::Running
                }
            }
        };

        if button_a_pressed == true {
            rprintln!("Low");
            timer1.delay_ms(1000 / FPS);
        } else {
        }

        display.show(&mut timer1, grid.grid, 1000 / FPS);
    }
}
