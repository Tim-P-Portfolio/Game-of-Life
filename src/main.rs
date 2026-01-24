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
use rtt_target::{debug_rprintln, debug_rtt_init_print};

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
}

#[entry]
fn main() -> ! {
    debug_rtt_init_print!();

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

    // Set buttons as input in pullup state
    let mut button_a = board.buttons.button_a.into_pullup_input();
    let mut button_b = board.buttons.button_b.into_pullup_input();

    // Initialize the counters
    let mut b_btn_frame_count = 5;
    let mut stall_frame_count = 1;
    let mut done_frame_count = 1;

    // Initialize state to start as random grid
    let mut state = GameState::Randomize;

    loop {
        // Get button states
        let button_a_pressed = button_a.is_low().unwrap();
        let button_b_pressed = button_b.is_low().unwrap();

        // Match state to operations
        state = match state {
            GameState::ButtonAPressed => {
                debug_rprintln!("Button A pressed");
                GameState::Randomize
            }
            GameState::ButtonBPressed => {
                debug_rprintln!("Button B pressed");

                b_btn_frame_count = 1;
                GameState::Complement
            }
            GameState::Randomize => {
                debug_rprintln!("~~ Randomize");
                past_grids.clear();
                grid.generate_random_grid(&mut rng);

                if button_a_pressed {
                    GameState::Randomize
                } else {
                    GameState::Running
                }
            }
            GameState::Complement => {
                debug_rprintln!("Complement");
                grid.complement();

                GameState::Running
            }
            GameState::Done => {
                debug_rprintln!(" ! Done: {}", done_frame_count);
                done_frame_count += 1;
                if done_frame_count > 5 {
                    done_frame_count = 1;
                    GameState::Randomize
                } else {
                    GameState::Done
                }
            }
            GameState::Running => {
                debug_rprintln!("> Running");
                if button_a_pressed {
                    GameState::ButtonAPressed
                } else if button_b_pressed && b_btn_frame_count > 5 {
                    GameState::ButtonBPressed
                } else if done(&grid.grid) {
                    GameState::Done
                } else {
                    life(&mut grid.grid);
                    if USING_STALL {
                        if past_grids.repeating() {
                            if stall_frame_count < STALL_DELAY_FRAMES {
                                debug_rprintln!(
                                    "            Stall count --> {}",
                                    stall_frame_count
                                );
                                stall_frame_count += 1;
                                GameState::Running
                            } else {
                                stall_frame_count = 1;
                                GameState::Randomize
                            }
                        } else {
                            past_grids.set(grid);

                            GameState::Running
                        }
                    } else {
                        GameState::Running
                    }
                }
            }
        };

        // Display the grid
        display.show(&mut timer1, grid.grid, 1000 / FPS);

        // Increment B button timeout
        if b_btn_frame_count <= 5 {
            debug_rprintln!("        Button B delay {}", b_btn_frame_count);
            b_btn_frame_count += 1;
        }
    }
}
