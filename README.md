# Game of Life
Game of Life on the Micro:Bit V2

### Features:
  + detect_stall : Detect stalls and randomize after 5 frames and 2 seconds 
  + print_microbit : Print ASCII microbit with starting state

### Assignment: **Life**
#### - _Tim Pup_

*A writeup of what I did, how it went, and other observations of interest.*

This project implements the game of life on a Micro:Bit V2.2. Using the display HAL (Hardware Access layer) and a provided game of life logic module.


The specs for the project were as follows:
- [x] The program runs the game at 10 frames per second (updates once per 100ms).
- [x] The program starts with a random board.
- [x] While the A button is held, the board is re-randomized every frame.
- [x] Otherwise, when the B button is not ignored and is pressed, the board is “complemented”: every “on” cell is turned “off” and every “off” cell is turned “on”. The B button is then ignored for 5 frames (0.5s).
- [x] Otherwise, if the program reaches a state where all cells on the board are off, the program waits 5 frames (0.5s). If it has not received a button press, it then starts with a new random board.
- [x] Otherwise, normal Life steps are taken.

Rather than having a done condition I chose to implement a stall condition. The stall condition checks for repeated grid states. When a grid state is repeated 5 times in a row the grid is randomized. This handels the done condition as well.

Problems incountered:
  + Initally for the random access I attempted to use the pariferial directly through the pac crate. This was difficult and did not end up working. I switched to using the HAL for access to the hardware random. This was much simpler to work with.
  + A fair bit of my time working on this project was spent on learing how to deal with strings without the use of the standard library. While completely unnececary

  
  
Cool thing with match statements
``` Rust
  GameState::Repeating { delay: d @ 2 } => GameState::Randomize,
  GameState::Repeating { delay: d @ 0..2 } => GameState::Repeating { delay: d + 1 },
```
