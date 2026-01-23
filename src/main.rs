#![no_main]
#![no_std]

mod grid;
use grid::*;

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

const FPS: u32 = 10;

// State of the game
enum GameState {
    ButtonAPressed,
    ButtonBPressed,
    Randomize,
    Running,
    Complement,
    Done,
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
    let mut past_grids = GridBuffer::new();
    // let mut past_grid = Grid::new();

    // Set buttons as input in pullup state
    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    // Initialize the counters
    let mut b_btn_frame_count = 5;

    // Initialize state to start as random grid
    let mut state = GameState::Randomize;

    let mut stall_frame_count = 1;

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
                rprintln!("Rand");
                grid.generate_random_grid(&mut rng);

                rprintln!("Rand");
                GameState::Running
            }
            GameState::Complement => {
                grid.complement();
                GameState::Running
            }
            GameState::Done => {
                rprintln!("Done");
                stall_frame_count += 1;
                if stall_frame_count > 5 {
                    stall_frame_count = 0;
                    GameState::Randomize
                } else {
                    GameState::Done
                }
            }
            GameState::Running => {
                rprintln!("Running real");
                if button_a_pressed {
                    rprintln!("Button A");
                    GameState::ButtonAPressed
                } else if button_b_pressed {
                    GameState::ButtonBPressed
                } else if done(&grid.grid) {
                    GameState::Done
                } else {
                    rprintln!("Running real 3");
                    if USING_STALL {
                        past_grids.set(grid);
                        rprintln!("{}", past_grids.repeat);
                    }

                    life(&mut grid.grid);
                    GameState::Running
                }
            }
        };

        rprintln!("Running {:?}", done(&mut grid.grid));
        // Display the grid
        display.show(&mut timer1, grid.grid, 1000 / FPS);

        // Increment B button timeout
        if b_btn_frame_count < 5 {
            b_btn_frame_count += 1
        }
    }
}
