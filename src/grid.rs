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
}
impl GridBuffer {
    pub fn new() -> Self {
        Self {
            grids: [Grid::new(); BUFFER_SIZE],
            top: 0,
        }
    }

    pub fn repeating(&self) -> bool {
        match self.top {
            0 => false,
            d if d % 2 == 0 => {
                let unique_len = self.top / 2;
                let mut repeated = true;
                let length: usize = self.top;
                for i in 0..length {
                    let l = i;
                    let r = length - i;
                    rprintln!("{}{}", l, r);
                    if self.grids[l].grid != self.grids[r].grid {
                        repeated = false;
                        rprintln!("Broken");
                        break;
                    }
                }
                repeated
            }
            _ => false,
        }
    }

    fn clear() {}

    pub fn set(&mut self, grid: Grid) {
        // This doesn't work at all
        // Need keep track
        //
        // Rules:
        // Never same twice in a row
        // Never streak of < 4 looping
        //
        // Repeats:
        // 1: 00, 00 ==> if last == current
        // 2: 01, 00, 01 ==> if 3rd == first
        // 3: 00, 01, 10, 00, 01, 10 ==> if 4th == 1st && 5th == 2st && 6th == 3rd <- should work for all
        //
        // 1st = last
        //
        // 3:   0..3                                     4..6
        // grid[0..max_repeat_len] = grid[max_repeat_len+1..len]
        //
        // 1: 2
        // 2: 4
        // 3: 6
        // : 2n
        //
        // Buffer len will always be even

        // Set top to top + 1 wrapping back to 0

        rprintln!();
        for r in 0..5 {
            for g in 0..self.top {
                for c in 0..5 {
                    rprint!("{}", self.grids[g].grid[r][c]);
                }
                rprint!("  ");
            }
            rprint!("\n");
        }
        rprintln!();

        // Add new grid to buffer
        self.grids[self.top].set(grid.grid);

        self.top = if self.top + 1 < BUFFER_SIZE {
            self.top + 1
        } else {
            0
        };
    }
}
