# Chip-8 Emulator

This is emulator is built in Rust, for the purpose of understanding/learning how to build an emulator.

## Setup
Install `sdl2`
```bash
brew install sdl2
```

## Commands
```bash
cargo run programs/tetris.c8
```

## Controls
Here is a list of the usable keys when playing a game in this emulator (controls will differ depending on the game).
```
1 2 3 4
q w e r
a s d f
z x c v
```

## Todos
1. Create a separate thread to process video.
2. Replace sdl2 with std or other libs.

## Resources
- [Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM)
- [Mattmikolay Chip-8 Technical Reference](https://github.com/mattmikolay/chip-8/wiki/CHIP%E2%80%908-Technical-Reference)
