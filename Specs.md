# Specs:

- [x] The program runs the game at 10 frames per second (updates once per 100ms).
- [x] The program starts with a random board.
- [x] While the A button is held, the board is re-randomized every frame.
- [x] Otherwise, when the B button is not ignored and is pressed, the board is “complemented”: every “on” cell is turned “off” and every “off” cell is turned “on”. The B button is then ignored for 5 frames (0.5s).
- [ ] Otherwise, if the program reaches a state where all cells on the board are off, the program waits 5 frames (0.5s). If it has not received a button press, it then starts with a new random board.
- [ ] Otherwise, normal Life steps are taken.
