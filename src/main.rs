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
use rtt_target::{rprintln as print, rtt_init_print};

const USING_STALL: bool = cfg!(feature = "detect_stall");
const STALL_DELAY_SECONDS: u8 = 2;

const FPS: u32 = 10;
const STALL_DELAY_FRAMES: u8 = STALL_DELAY_SECONDS * FPS as u8;

// State of the game
enum GameState {
    ButtonAPressed,
    ButtonBPressed,
    Randomize,
    Running,
    Complement,
    Done,
    Repeating { delay: u8 },
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
                grid.generate_random_grid(&mut rng);

                GameState::Running
            }
            GameState::Complement => {
                grid.complement();
                GameState::Running
            }
            GameState::Done => {
                stall_frame_count += 1;
                if stall_frame_count > 5 {
                    stall_frame_count = 0;
                    GameState::Randomize
                } else {
                    GameState::Done
                }
            }
            GameState::Running => {
                if button_a_pressed {
                    GameState::ButtonAPressed
                } else if button_b_pressed {
                    GameState::ButtonBPressed
                } else if done(&grid.grid) {
                    GameState::Done
                } else {
                    life(&mut grid.grid);
                    if USING_STALL {
                        past_grids.set(grid);
                        if past_grids.repeating() {
                            print!("repeated");
                            GameState::Repeating { delay: 0 }
                        } else {
                            GameState::Running
                        }
                    } else {
                        GameState::Running
                    }
                }
            }
            GameState::Repeating { delay: d } if d >= STALL_DELAY_FRAMES => {
                stall_frame_count = 0;
                GameState::Randomize
            }
            GameState::Repeating { delay: d @ _ } => {
                print!("waiting");
                GameState::Repeating { delay: d + 1 }
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
