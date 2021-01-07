//! CHIP-8 is an interpreted programming language. Chip-8 programs are typically run on CHIP-8
//! virtual machines

#![deny(missing_docs)]

use rand::prelude::*;

const CHIP8_WIDTH: usize = 64;
const CHIP8_HEIGHT: usize = 32;

/// This struct represents the CPU structure of CHIP-8 systems
pub struct Chip8 {
    /// Index register (0x000-0xFFF)
    i: u16,
    /// Program counter (0x000-0xFFF)
    pc: u16,
    /// Stack pointer
    sp: u8,
    /// Delay timer register
    dt: u8,
    /// Sound timer register
    st: u8,

    /// Fifteen 8-bit general purpose registers, the 16th register is used as a "carry flag"
    v: [u8; 16],
    /// 4K memory
    /// - (0, 512): Chip 8 interpreter (contains font set in emulator)
    /// - (512, 4096): Chip 8 program
    mem: [u8; 4096],
    /// Sixteen-level stack
    stack: [u16; 16],

    /// Graphics system, one instruction is used the draw sprite to the
    /// screen; drawing is done in XOR mode, VF register is set if a
    /// pixel is turned off.
    gfx: [bool; CHIP8_WIDTH * CHIP8_HEIGHT],
    /// Current state of the HEX-based keypad
    key: [bool; 16],
}

impl Default for Chip8 {
    fn default() -> Self {
        Self {
            i: 0,
            pc: 0,
            sp: 0,
            dt: 0,
            st: 0,
            v: [0; 16],
            mem: [0; 4096],
            stack: [0; 16],
            gfx: [false; CHIP8_WIDTH * CHIP8_HEIGHT],
            key: [false; 16],
        }
    }
}

impl Chip8 {
    /// Set the state of the system to the intial state
    pub fn reset(&mut self) {
        self.i = 0;
        self.pc = 0x200; // program begins at 0x200
        self.sp = 0;
        self.dt = 0;
        self.st = 0;

        self.v = [0; 16];
        self.mem = [0; 4096];
        self.stack = [0; 16];
        // load font sprites to the first 80 bytes of the memory
        FONTSET
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.mem[i] = *b);

        self.gfx = [false; CHIP8_WIDTH * CHIP8_HEIGHT]; // clear display
        self.key = [false; 16]; // clear display
    }

    /// Run one system clock cycle
    pub fn cycle(&mut self) {
        // Fetch opcode at the memory location specified by the program counter
        // The opcode is 2-byte long, so we fetch 2 consecutive bytes from the
        // memory and merge them.
        let pc = self.pc as usize;
        let opcode = (self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16;

        // Decode and execute opcode
        self.exec(opcode);

        // Update timers
        // The two timers count down to zero if they have been set to a
        // value larger than zero (counting at 60Hz).
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.st > 0 {
            if self.st == 1 {
                println!("BEEP");
            }
            self.st -= 1;
        }
    }

    fn exec(&mut self, opcode: u16) {
        let opcode_n0 = (opcode & 0xF000) >> 12;
        let opcode_n1 = (opcode & 0x0F00) >> 8;
        let opcode_n2 = (opcode & 0x00F0) >> 4;
        let opcode_n3 = opcode & 0x000F;

        let x = opcode_n1 as usize;
        let y = opcode_n2 as usize;
        let n = opcode & 0x000F;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (opcode_n0, opcode_n1, opcode_n2, opcode_n3) {
            // 00E0 - CLS
            // Clear the display.
            (0x0, 0x0, 0xE, 0x0) => self.gfx.iter_mut().for_each(|pixel| *pixel = false),

            // 00EE - RET
            // Return from a subroutine.
            // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            (0x0, 0x0, 0xE, 0xE) => {
                self.pc = self.stack[self.sp as usize];
                self.sp -= 1;
            }

            // 1NNN - JP addr
            // Jump to location nnn.
            // The interpreter sets the program counter to nnn.
            (0x1, _, _, _) => self.pc = nnn,

            // 2NNN - CALL addr
            // Call subroutine at nnn.
            // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            (0x2, _, _, _) => {
                self.stack[self.sp as usize] = self.pc;
                self.sp += 1;
                self.pc = nnn;
            }

            // 3XKK - SE Vx, byte
            // Skip next instruction if Vx = kk.
            // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            (0x3, _, _, _) => self.pc += if self.v[x] == kk { 2 } else { 0 },

            // 4XKK - SNE Vx, byte
            // Skip next instruction if Vx != kk.
            // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            (0x4, _, _, _) => self.pc += if self.v[x] != kk { 2 } else { 0 },

            // 5XY0 - SE Vx, Vy
            // Skip next instruction if Vx = Vy.
            // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            (0x5, _, _, 0x0) => self.pc += if self.v[x] == self.v[y] { 2 } else { 0 },

            // 6XKK - LD Vx, byte
            // Set Vx = kk.
            // The interpreter puts the value kk into register Vx.
            (0x6, _, _, _) => self.v[x] = kk,

            // 7XKK - ADD Vx, byte
            // Set Vx = Vx + kk.
            // Adds the value kk to the value of register Vx, then stores the result in Vx.
            (0x7, _, _, _) => self.v[x] += kk,

            // 8XY0 - LD Vx, Vy
            // Set Vx = Vy.
            // Stores the value of register Vy in register Vx.
            (0x8, _, _, 0x0) => self.v[x] = self.v[y],

            // 8XY1 - OR Vx, Vy
            // Set Vx = Vx OR Vy.
            // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits
            // from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
            (0x8, _, _, 0x1) => self.v[x] |= self.v[y],

            // 8XY2 - AND Vx, Vy
            // Set Vx = Vx AND Vy.
            // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits
            // from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
            (0x8, _, _, 0x2) => self.v[x] &= self.v[y],

            // 8XY3 - XOR Vx, Vy
            // Set Vx = Vx XOR Vy.
            // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares
            // the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result
            // is set to 1. Otherwise, it is 0.
            (0x8, _, _, 0x3) => self.v[x] ^= self.v[y],

            // 8XY4 - ADD Vx, Vy
            // Set Vx = Vx + Vy, set VF = carry.
            // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0.
            // Only the lowest 8 bits of the result are kept, and stored in Vx.
            (0x8, _, _, 0x4) => {
                let (res, overflow) = self.v[x].overflowing_add(self.v[y]);
                self.v[0xF] = if overflow { 1 } else { 0 };
                self.v[x] = res
            }

            // 8XY5 - SUB Vx, Vy
            // Set Vx = Vx - Vy, set VF = NOT borrow.
            // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
            (0x8, _, _, 0x5) => {
                let (res, overflow) = self.v[x].overflowing_sub(self.v[y]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res
            }

            // 8XY6 - SHR Vx {, Vy}
            // Set Vx = Vx SHR 1.
            // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            (0x8, _, _, 0x6) => {
                self.v[0xF] = self.v[x] & 0x01;
                self.v[x] >>= 1;
            }

            // 8XY7 - SUBN Vx, Vy
            // Set Vx = Vy - Vx, set VF = NOT borrow.
            // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
            (0x8, _, _, 0x7) => {
                let (res, overflow) = self.v[y].overflowing_sub(self.v[x]);
                self.v[0xF] = if overflow { 0 } else { 1 };
                self.v[x] = res
            }

            // 8XYE - SHL Vx {, Vy}
            // Set Vx = Vx SHL 1.
            // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            (0x8, _, _, 0xE) => {
                self.v[0xF] = self.v[x] & 0x80;
                self.v[x] <<= 1;
            }

            // 9XY0 - SNE Vx, Vy
            // Skip next instruction if Vx != Vy.
            // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            (0x9, _, _, 0x0) => self.pc += if self.v[x] != self.v[y] { 2 } else { 0 },

            // ANNN - ld i, addr
            // set i = nnn.
            // the value of register i is set to nnn.
            (0xA, _, _, _) => self.i = nnn,

            // BNNN - JP V0, addr
            // Jump to location nnn + V0.
            // The program counter is set to nnn plus the value of V0.
            (0xB, _, _, _) => self.pc = self.v[0] as u16 + nnn,

            // CXKK - RND Vx, byte
            // Set Vx = random byte AND kk.
            // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx.
            // See instruction 8xy2 for more information on AND.
            (0xC, _, _, _) => self.v[x] = random::<u8>() & kk,

            // Dxyn - DRW Vx, Vy, nibble
            // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
            // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprite
            // on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased,
            // VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display,
            // it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4,
            // Display, for more information on the Chip-8 screen and sprites.
            (0xD, _, _, _) => {
                self.v[0xF] = 0;
                for (y_offset, sprite) in self.mem[self.i as usize..(self.i + n as u16) as usize]
                    .iter()
                    .enumerate()
                {
                    for x_offset in 0..8 {
                        if (sprite & (0x80 >> x_offset)) != 0 {
                            if self.gfx[x + x_offset + (y + y_offset) * CHIP8_WIDTH] {
                                self.v[0xF] = 1;
                            }
                            self.gfx[x + x_offset + (y + y_offset) * CHIP8_WIDTH] ^= true;
                        }
                    }
                }
            }

            // Ex9E - SKP Vx
            // Skip next instruction if key with the value of Vx is pressed.
            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
            (0xE, _, 0x9, 0xE) => self.pc += if self.key[self.v[x] as usize] { 2 } else { 0 },

            // ExA1 - SKNP Vx
            // Skip next instruction if key with the value of Vx is not pressed.
            // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
            (0xE, _, 0xA, 0x1) => self.pc += if self.key[self.v[x] as usize] { 0 } else { 2 },

            // Fx07 - LD Vx, DT
            // Set Vx = delay timer value.
            // The value of DT is placed into Vx.
            (0xF, _, 0x0, 0x7) => self.v[x] = self.dt,

            // Fx0A - LD Vx, K
            // Wait for a key press, store the value of the key in Vx.
            // All execution stops until a key is pressed, then the value of that key is stored in Vx.
            (0xF, _, 0x0, 0xA) => {
                self.pc -= 2; // reset program counter to previous position
                for (key_idx, key_pressed) in self.key.iter().enumerate() {
                    if *key_pressed {
                        self.v[x] = key_idx as u8;
                        self.pc += 2;
                    }
                }
            }

            // Fx15 - LD DT, Vx
            // Set delay timer = Vx.
            // DT is set equal to the value of Vx.
            (0xF, _, 0x1, 0x5) => self.dt = self.v[x],

            // Fx18 - LD ST, Vx
            // Set sound timer = Vx.
            // ST is set equal to the value of Vx.
            (0xF, _, 0x1, 0x8) => self.st = self.v[x],

            // Fx1E - ADD I, Vx
            // Set I = I + Vx.
            // The values of I and Vx are added, and the results are stored in I.
            (0xF, _, 0x1, 0xE) => self.i += self.v[x] as u16,

            // Fx29 - LD F, Vx
            // Set I = location of sprite for digit Vx.
            // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display,
            // for more information on the Chip-8 hexadecimal font.
            (0xF, _, 0x2, 0x9) => self.i = self.v[x] as u16 * 5, // each hexadecimal sprite is 5-byte long

            // Fx33 - LD B, Vx
            // Store BCD representation of Vx in memory locations I, I+1, and I+2.
            // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit
            // at location I+1, and the ones digit at location I+2.
            (0xF, _, 0x3, 0x3) => {
                self.mem[self.i as usize] = self.v[x] / 100;
                self.mem[self.i as usize + 1] = (self.v[x] / 10) % 10;
                self.mem[self.i as usize + 2] = (self.v[x] % 100) % 10;
            }

            // Fx55 - LD [I], Vx
            // Store registers V0 through Vx in memory starting at location I.
            // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
            (0xF, _, 0x5, 0x5) => self.mem[self.i as usize..self.i as usize + x + 1]
                .copy_from_slice(&self.v[0..x + 1]),

            // Fx65 - LD Vx, [I]
            // Read registers V0 through Vx from memory starting at location I.
            // The interpreter reads values from memory starting at location I into registers V0 through Vx.
            (0xF, _, 0x6, 0x5) => self.v[0..x + 1]
                .copy_from_slice(&self.mem[self.i as usize..self.i as usize + x + 1]),

            // Unsupported opcode
            (_, _, _, _) => {
                todo!()
            }
        }
    }
}

/// The first four nibble is used to determine what the character,
const FONTSET: [u8; 80] = [
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

#[cfg(test)]
mod tests {}
