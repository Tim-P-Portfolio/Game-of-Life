#![no_main]
#![no_std]

mod life;
use life::*;

use embedded_hal::digital::InputPin;

use cortex_m_rt::entry;
use microbit::{
    Board,
    display::blocking::Display,
    hal::{rng::Rng, timer::Timer},
};
use panic_halt as _;
use rtt_target::{rprint, rprintln, rtt_init_print};

const USING_STALL: bool = cfg!(feature = "detect_stall");

// Wether or not to display the text microbit
const PRINT_MICROBIT: bool = cfg!(feature = "print_microbit");

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

// Grid for setting the state of the display leds
#[derive(Copy, Clone)]
struct Grid {
    grid: [[u8; 5]; 5],
}

impl Grid {
    fn new() -> Self {
        // Initialize 5x5 grid with zeros
        Self { grid: [[0; 5]; 5] }
    }

    // Flip all grid states
    fn complement(&mut self) {
        for r in 0..5 {
            for c in 0..5 {
                self.grid[r][c] = 1 - self.grid[r][c]
            }
        }
    }

    fn set(&mut self, grid: [[u8; 5]; 5]) {
        for r in 0..5 {
            for c in 0..5 {
                self.grid[r][c] = grid[r][c]
            }
        }
    }

    fn generate_random_grid(&mut self, rng: &mut Rng) {
        // Display first 3 lines of the text based microbit in the terminal
        if PRINT_MICROBIT {
            for i in 0..4 {
                rprintln!("\r{}", MICROBIT[i]);
            }
        }
        // Loop through grid cells
        for c in 0..5 {
            // Print edge and spacing for microbit
            if PRINT_MICROBIT {
                rprint!("\r│      ")
            }
            for r in 0..5 {
                // Set led state to random value, 0-127: 0, 128-255: 1
                let num = if rng.random_u8() > 127 { 1 } else { 0 };
                // Set microbit
                self.grid[c][r] = num;
                // Print text version of starting state of leds
                if PRINT_MICROBIT {
                    rprint!("{}", if num == 1 { " ▮" } else { " ▯" })
                }
            }
            // Print edge and spacing for microbit
            if PRINT_MICROBIT {
                rprint!("       │\n");
            }
        }

        // Display last 3 lines of the microbit in the terminal
        if PRINT_MICROBIT {
            for i in 5..7 {
                rprintln!("\r{}", MICROBIT[i]);
            }
        }
    }
}

// State of the game
enum GameState {
    ButtonAPressed,
    ButtonBPressed,
    Randomize,
    Running,
    Complement,
    // Done,
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    // Setup microbit board
    let board = Board::take().unwrap();

    // Setup timer
    let mut timer1 = Timer::new(board.TIMER1);

    // Setup microbit display
    let mut display = Display::new(board.display_pins);

    // Setup hardware RNG
    let mut rng = Rng::new(board.RNG);

    // Initialize a grid
    let mut grid = Grid::new();
    // Grid to check for stall
    // let mut past_grids = [Grid::new(); 3];
    let mut past_grid = Grid::new();

    // Set buttons as input in pullup state
    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    // Initialize the counters
    let mut b_btn_frame_count = 5;
    let mut stall_frame_count = 5;
    // let mut done_frame_count = 1;

    // Initialize state to start as random grid
    let mut state = GameState::Randomize;

    loop {
        // Get button states
        let button_a_pressed = button_a.is_low().unwrap();
        let button_b_pressed = button_b.is_low().unwrap();

        // Match state to operations
        state = match state {
            GameState::ButtonAPressed => GameState::Randomize,
            GameState::ButtonBPressed => {
                if b_btn_frame_count < 5 {
                    GameState::Running
                } else {
                    b_btn_frame_count = 1;
                    GameState::Complement
                }
            }
            GameState::Randomize => {
                grid.generate_random_grid(&mut rng);
                GameState::Running
            }
            GameState::Complement => {
                grid.complement();
                GameState::Running
            }
            GameState::Running => {
                if button_a_pressed {
                    GameState::ButtonAPressed
                } else if button_b_pressed {
                    GameState::ButtonBPressed
                // } else if done(&grid.grid) {
                //     GameState::Done
                } else {
                    if past_grid.grid == grid.grid {
                        stall_frame_count += 1
                    } else {
                        past_grid.set(grid.grid);
                        stall_frame_count = 1
                    }
                    // Run life proceedure on grid
                    life(&mut grid.grid);
                    // When:
                    //      done state has lasted 5 frames
                    //     or ( with detect stall enabled )
                    //      stalled for 5 frames and 2 seconds extra
                    // randomize the board
                    if (stall_frame_count > (5 + (FPS * 2)) && USING_STALL)
                        || (done(&grid.grid) && stall_frame_count > 5)
                    {
                        GameState::Randomize
                    } else {
                        GameState::Running
                    }
                }
            }
        };

        // Display the grid
        display.show(&mut timer1, grid.grid, 1000 / FPS);

        // Increment B button timeout
        if b_btn_frame_count < 5 {
            b_btn_frame_count += 1
        }
    }
}
