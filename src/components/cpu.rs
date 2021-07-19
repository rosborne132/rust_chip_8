use rand;
use std::fs::File;
use std::io::prelude::*;

use super::constants::FONTSET;

pub struct Cpu {
    pc: u16,
    opcode: u16,
    ir: u16,
    sp: u16,

    v: [u8; 16],
    stack: [u16; 16],
    memory: [u8; 4096],

    pub gfx: [u8; 2048],
    pub key: [u8; 16],

    delay_timer: u8,
    sound_timer: u8,

    pub draw_flag: bool,
    pub beep_flag: bool,
}

impl Cpu {
    pub fn new() -> Self {
        let mut cpu = Self {
            pc: 0x200, // starts at 0x200
            opcode: 0,
            ir: 0,
            sp: 0,

            v: [0; 16],
            stack: [0; 16],
            memory: [0; 4096],

            gfx: [0; 2048],
            key: [0; 16],

            delay_timer: 0,
            sound_timer: 0,

            draw_flag: true,
            beep_flag: false,
        };

        // load fontset
        for i in 0..80 {
            cpu.memory[i] = FONTSET[i];
        }

        return cpu;
    }

    pub fn opcode(&mut self) {
        // fetch opcode
        self.opcode = (self.memory[self.pc as usize] as u16) << 8
            | (self.memory[(self.pc + 1) as usize] as u16);

        // For troubleshooting opcodes
        // println!("opcode: {:02X}{:02X}", (self.opcode >> 8) as u8, self.opcode as u8);

        // process opcode
        match self.opcode & 0xF000 {
            0x0000 => {
                match self.opcode & 0x000F {
                    // 0x00E0: clears the screen
                    0x0000 => {
                        for i in 0..2048 {
                            self.gfx[i] = 0;
                        }
                        self.draw_flag = true;
                        self.pc += 2;
                    }
                    // 0x00EE: returns from subroutine
                    0x000E => {
                        self.sp -= 1; // 16 levels of stack, decrease stack pointer to prevent overwrite
                        self.pc = self.stack[self.sp as usize]; // put the stored return address from the stack back into the program counter
                        self.pc += 2 // don't forget to increase the program counter!
                    }
                    _ => {
                        panic!("unknown opcode [0x0000]: 0x{:X}.", self.opcode);
                    }
                }
            }

            // 0x1NNN: jumps to address NNN
            0x1000 => {
                self.pc = self.opcode & 0x0FFF;
            }

            // 0x2NNN: calls subroutine at NNN.
            0x2000 => {
                self.stack[self.sp as usize] = self.pc; // store current address in stack
                self.sp += 1; // increment stack pointer
                self.pc = self.opcode & 0x0FFF; // set the program counter to the address at NNN
            }

            // 0x2NNN: calls subroutine at NNN.
            0x3000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] == (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 0x4XNN: skips the next instruction if VX doesn't equal NN
            0x4000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize] != (self.opcode & 0x00FF) as u8 {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 0x5XY0: skips the next instruction if VX equals VY
            0x5000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    == self.v[((self.opcode & 0x00F0) >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // 0x6XNN: sets VX to NN
            0x6000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] = (self.opcode & 0x00FF) as u8;
                self.pc += 2;
            }

            // 0x7XNN: adds NN to VX
            0x7000 => {
                let pos: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                self.v[pos] = self.v[pos].wrapping_add((self.opcode & 0x00FF) as u8);
                self.pc += 2;
            }

            0x8000 => {
                match self.opcode & 0x000F {
                    // 0x8XY0: sets VX to the value of VY
                    0x0000 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] =
                            self.v[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    }

                    // 0x8XY1: sets VX to "VX OR VY"
                    0x0001 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] |=
                            self.v[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    }

                    // 0x8XY2: sets VX to "VX AND VY"
                    0x0002 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] &=
                            self.v[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    }

                    // 0x8XY3: sets VX to "VX XOR VY"
                    0x0003 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] ^=
                            self.v[((self.opcode & 0x00F0) >> 4) as usize];
                        self.pc += 2;
                    }

                    // 0x8XY4: adds VY to VX. VF is set to 1 when there's a carry, and to 0 when there isn't
                    0x0004 => {
                        if self.v[((self.opcode & 0x00F0) >> 4) as usize]
                            > (0xFF - self.v[((self.opcode & 0x0F00) >> 8) as usize])
                        {
                            self.v[0xF] = 1; // carry
                        } else {
                            self.v[0xF] = 0;
                        }
                        let pos: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        self.v[pos] = self.v[pos]
                            .wrapping_add(self.v[((self.opcode & 0x00F0) >> 4) as usize]);
                        self.pc += 2;
                    }

                    // 0x8XY5: VY is subtracted from VX. VF is set to 0 when there's a borrow, and 1 when there isn't
                    0x0005 => {
                        let pos: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        if self.v[((self.opcode & 0x00F0) >> 4) as usize] > self.v[pos] {
                            self.v[0xF] = 0; // there is a borrow
                        } else {
                            self.v[0xF] = 1;
                        }
                        self.v[pos] = self.v[pos]
                            .wrapping_sub(self.v[((self.opcode & 0x00F0) >> 4) as usize]);
                        self.pc += 2;
                    }

                    // 0x8XY6: shifts VX right by one. VF is set to the value of the least significant bit of VX before the shift
                    0x0006 => {
                        self.v[0xF] = self.v[((self.opcode & 0x0F00) >> 8) as usize] & 0x1;
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] >>= 1;
                        self.pc += 2;
                    }

                    // 0x8XY7: sets VX to VY minus VX. VF is set to 0 when there's a borrow, and 1 when there isn't
                    0x0007 => {
                        let pos: usize = ((self.opcode & 0x0F00) >> 8) as usize;
                        if self.v[pos] > self.v[((self.opcode & 0x00F0) >> 4) as usize] {
                            // VY-VX
                            self.v[0xF] = 0; // there is a borrow
                        } else {
                            self.v[0xF] = 1;
                        }
                        self.v[pos] = self.v[((self.opcode & 0x00F0) >> 4) as usize]
                            .wrapping_sub(self.v[pos]);
                        self.pc += 2;
                    }

                    // 0x8XYE: shifts VX left by one. VF is set to the value of the most significant bit of VX before the shift
                    0x000E => {
                        self.v[0xF] = self.v[((self.opcode & 0x0F00) >> 8) as usize] >> 7;
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] <<= 1;
                        self.pc += 2;
                    }

                    _ => {
                        panic!("unknown opcode [0x8000]: 0x{:X}.", self.opcode);
                    }
                }
            }

            // 0x9XY0: skips the next instruction if VX doesn't equal VY
            0x9000 => {
                if self.v[((self.opcode & 0x0F00) >> 8) as usize]
                    != self.v[((self.opcode & 0x00F0) >> 4) as usize]
                {
                    self.pc += 4;
                } else {
                    self.pc += 2;
                }
            }

            // ANNN: sets I to the address NNN
            0xA000 => {
                self.ir = self.opcode & 0x0FFF;
                self.pc += 2;
            }

            // BNNN: jumps to the address NNN plus V0
            0xB000 => {
                self.pc = (self.opcode & 0x0FFF).wrapping_add(self.v[0] as u16);
            }

            // CXNN: sets VX to a random number and NN
            0xC000 => {
                self.v[((self.opcode & 0x0F00) >> 8) as usize] =
                    rand::random::<u8>() & (self.opcode as u8);
                self.pc += 2;
            }

            // DXYN: draws a sprite at coordinate (VX, VY) that has a width of 8 pixels and a height of N pixels.
            // each row of 8 pixels is read as bit-coded starting from memory location ri;
            // ri value doesn't change after the execution of this instruction.
            // VF is set to 1 if any screen pixels are flipped from set to unset when the sprite is drawn,
            // and to 0 if that doesn't happen
            0xD000 => {
                let x = self.v[((self.opcode & 0x0F00) >> 8) as usize] as u16;
                let y = self.v[((self.opcode & 0x00F0) >> 4) as usize] as u16;
                let height = self.opcode & 0x000F;

                self.v[0xF] = 0;
                for yline in 0..height {
                    let pixel = self.memory[(self.ir + yline) as usize] as u16;
                    for xline in 0..8 {
                        if (pixel & (0x80 >> xline)) != 0 {
                            let pos = (x + xline + ((y + yline) * 64)) as usize;
                            if pos < 2048 {
                                if self.gfx[pos] == 1 {
                                    self.v[0xF] = 1;
                                }
                                self.gfx[pos] ^= 1;
                            }
                        }
                    }
                }

                self.draw_flag = true;
                self.pc += 2;
            }

            0xE000 => {
                match self.opcode & 0x00FF {
                    // EX9E: skips the next instruction if the key stored in VX is pressed
                    0x009E => {
                        if self.key[self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize] != 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    // EXA1: skips the next instruction if the key stored in VX isn't pressed
                    0x00A1 => {
                        if self.key[self.v[((self.opcode & 0x0F00) >> 8) as usize] as usize] == 0 {
                            self.pc += 4;
                        } else {
                            self.pc += 2;
                        }
                    }

                    _ => {
                        panic!("unknown opcode [0xE000]: 0x{:X}.", self.opcode);
                    }
                }
            }

            0xF000 => {
                match self.opcode & 0x00FF {
                    // FX07: sets VX to the value of the delay timer
                    0x0007 => {
                        self.v[((self.opcode & 0x0F00) >> 8) as usize] = self.delay_timer;
                        self.pc += 2;
                    }

                    // FX0A: a key press is awaited, and then stored in VX
                    0x000A => {
                        let mut key_press = false;

                        for i in 0..16 {
                            if self.key[i] != 0 {
                                self.v[((self.opcode & 0x0F00) >> 8) as usize] = i as u8;
                                key_press = true;
                            }
                        }

                        // if we didn't received a keypress, skip this cycle and try again.
                        if key_press {
                            self.pc += 2;
                        }
                    }

                    // FX15: sets the delay timer to VX
                    0x0015 => {
                        self.delay_timer = self.v[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    }

                    // FX18: sets the sound timer to VX
                    0x0018 => {
                        self.sound_timer = self.v[((self.opcode & 0x0F00) >> 8) as usize];
                        self.pc += 2;
                    }

                    // FX1E: adds VX to ir
                    0x001E => {
                        let sum = self
                            .ir
                            .wrapping_add(self.v[((self.opcode & 0x0F00) >> 8) as usize] as u16);
                        // VF is set to 1 when range overflow (I+VX>0xFFF), and 0 when there isn't
                        if sum > 0xFFF {
                            self.v[0xF] = 1;
                        } else {
                            self.v[0xF] = 0;
                        }
                        self.ir = sum;
                        self.pc += 2;
                    }

                    // FX29: sets ir to the location of the sprite for the character in VX. Characters 0-F (in hexadecimal) are represented by a 4x5 font
                    0x0029 => {
                        self.ir = self.v[((self.opcode & 0x0F00) >> 8) as usize] as u16 * 0x5;
                        self.pc += 2;
                    }

                    // FX33: stores the binary-coded decimal representation of VX at the addresses ir, ir plus 1, and ir plus 2
                    0x0033 => {
                        self.memory[self.ir as usize] =
                            self.v[((self.opcode & 0x0F00) >> 8) as usize] / 100;
                        self.memory[(self.ir + 1) as usize] =
                            (self.v[((self.opcode & 0x0F00) >> 8) as usize] / 10) % 10;
                        self.memory[(self.ir + 2) as usize] =
                            (self.v[((self.opcode & 0x0F00) >> 8) as usize] % 100) % 10;
                        self.pc += 2;
                    }

                    // FX55: stores V0 to VX in memory starting at address ir
                    0x0055 => {
                        let j = (self.opcode & 0x0F00) >> 8;
                        for i in 0..j + 1 {
                            self.memory[(self.ir + i) as usize] = self.v[i as usize];
                        }

                        // on the original interpreter, when the operation is done, ir = ir + X + 1.
                        self.ir = self.ir.wrapping_add(j + 1);
                        self.pc += 2;
                    }

                    // FX65: fills V0 to VX with values from memory starting at address ir
                    0x0065 => {
                        let j = (self.opcode & 0x0F00) >> 8;
                        for i in 0..j + 1 {
                            self.v[i as usize] = self.memory[(self.ir + i) as usize];
                        }

                        // on the original interpreter, when the operation is done, ir = ir + X + 1.
                        self.ir = self.ir.wrapping_add(j + 1);
                        self.pc += 2;
                    }

                    _ => {
                        panic!("unknown opcode [0xF000]: 0x{:X}.", self.opcode);
                    }
                }
            }

            _ => {
                panic!("unknown opcode [0x0000]: 0x{:X}.", self.opcode);
            }
        }

        // update timers
        if self.delay_timer > 0 {
            self.delay_timer -= 1;
        }

        if self.sound_timer > 0 {
            if self.sound_timer == 1 {
                self.beep_flag = true;
            }
            self.sound_timer -= 1;
        }
    }

    /*pub fn debug_render(& self) {
        // draw
        for y in 0..32 {
            for x in 0..64 {
                if self.gfx[((y * 64) + x) as usize] == 0 {
                    print!("O");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("");
    }*/

    pub fn load_application(&mut self, filename: &str) -> bool {
        // open file
        let mut file = File::open(filename).expect("Error occurred when opening file");

        // get file size
        let fsize = file.metadata().unwrap().len();

        // read the file to a buffer
        let mut buffer = vec![];
        file.read_to_end(&mut buffer).expect("Couldn't read file");
        drop(file);

        // copy the buffer to chip8 memory
        if (4096 - 512) > fsize {
            for i in 0..fsize {
                self.memory[(i + 512) as usize] = buffer[i as usize];
            }
        } else {
            panic!("ROM too big for memory");
        }

        return true;
    }
}
