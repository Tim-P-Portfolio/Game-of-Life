use core::str;

use rtt_target::{rprint, rprintln};

use microbit::hal::rng::Rng;

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

// Grid for setting the state of the display leds
#[derive(Copy, Clone)]
pub struct Grid {
    pub grid: [[u8; 5]; 5],
}

impl Grid {
    pub fn new() -> Self {
        // Initialize 5x5 grid with zeros
        Self { grid: [[0; 5]; 5] }
    }

    pub fn get(self) -> Grid {
        self
    }

    // Flip all grid states
    pub fn complement(&mut self) {
        for r in 0..5 {
            for c in 0..5 {
                self.grid[r][c] = 1 - self.grid[r][c]
            }
        }
    }

    pub fn set(&mut self, grid: [[u8; 5]; 5]) {
        for r in 0..5 {
            for c in 0..5 {
                self.grid[r][c] = grid[r][c]
            }
        }
    }

    pub fn generate_random_grid(&mut self, rng: &mut Rng) {
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

const MIN_UNIQUE: usize = 4;
const BUFFER_SIZE: usize = (MIN_UNIQUE - 1) * 2;

/*
 * Grid Buffer
 *
 * grids: list of grids
 * top: most recent grid
 *
 * Insert new grid at top+1
 * Set top to new grid
 */
pub struct GridBuffer {
    pub grids: [Grid; BUFFER_SIZE],
    pub top: usize,
    pub stalled: bool,
}
impl GridBuffer {
    pub fn new() -> Self {
        Self {
            grids: [Grid::new(); BUFFER_SIZE],
            top: 0,
            stalled: false,
        }
    }

    pub fn repeating(&mut self) -> bool {
        if self.top >= BUFFER_SIZE {
            self.top = 0
        }

        match self.top {
            0 => self.stalled,
            d if d % 2 == 0 => {
                self.stalled = true;
                let length: usize = self.top / 2;

                for i in 0..length {
                    let l = i;
                    let r = length - i;
                    if self.grids[l].grid != self.grids[r].grid {
                        self.stalled = false;
                        break;
                    }
                }
                if self.stalled {
                    self.top = 0
                }
                self.stalled
            }
            _ => self.stalled,
        }
    }

    pub fn clear(&mut self) {
        self.top = 0;
        self.stalled = false
    }

    pub fn set(&mut self, grid: Grid) {
        // Add new grid to buffer
        self.grids[self.top].set(grid.grid);

        // Increment top
        self.top = if self.top + 1 < BUFFER_SIZE {
            self.top + 1
        } else {
            0
        };
    }
}
