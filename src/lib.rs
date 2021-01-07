//! CHIP-8 is an interpreted programming language. Chip-8 programs are typically run on CHIP-8
//! virtual machines

#![deny(missing_docs)]

/// The first four nibble is used to determine what the character,
const CHIP8_FONTSET: [u8; 80] = [
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
    /// Hardware timer register
    ht: u8,

    /// Fifteen 8-bit general purpose registers, the 16th register is used as a "carry flag"
    v: [u8; 16],
    /// Sixteen-level stack
    stack: [u16; 16],
    /// 4K memory
    /// - (0, 512): Chip 8 interpreter (contains font set in emulator)
    /// - (512, 4096): Chip 8 program
    mem: [u8; 4096],

    /// Graphics system, one instruction is used the draw sprite to the
    /// screen; drawing is done in XOR mode, VF register is set if a
    /// pixel is turned off.
    gfx: [u8; 2048],
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
            ht: 0,
            v: [0; 16],
            stack: [0; 16],
            mem: [0; 4096],
            gfx: [0; 2048],
            key: [false; 16],
        }
    }
}

impl Chip8 {
    /// Set the state of the system to the intial state
    pub fn reset(&mut self) {
        self.i = 0; // reset index register
        self.pc = 0x200; // pc starts at 0x200 (the beginning of the program)
        self.sp = 0; // reset stack pointer
        self.dt = 0; // reset delay timer
        self.ht = 0; // reset hardware timer

        self.v = [0; 16]; // clear registers
        self.stack = [0; 16]; // clear memory
        self.mem = [0; 4096];
        CHIP8_FONTSET
            .iter()
            .enumerate()
            .for_each(|(i, b)| self.mem[i] = *b);

        self.gfx = [0; 2048]; // clear display
        self.key = [false; 16]; // clear display
    }

    /// Run one system clock cycle
    pub fn cycle(&mut self) {
        // Fetch opcode at the memory location specified by the program counter
        // The opcode is 2-byte long, so we fetch 2 consecutive bytes from the
        // memory and merge them.
        let pc = self.pc as usize;
        let opcode = (self.mem[pc] as u16) << 8 | self.mem[pc + 1] as u16;
        self.exec(opcode);

        // Update timers
        // The two timers count down to zero if they have been set to a
        // value larger than zero (counting at 60Hz).
        if self.dt > 0 {
            self.dt -= 1;
        }

        if self.ht > 0 {
            if self.ht == 1 {
                println!("BEEP");
            }
            self.ht -= 1;
        }
    }

    fn exec(&mut self, opcode: u16) {
        let opcode_b0 = (opcode & 0xF000) >> 12;
        let opcode_b1 = (opcode & 0x0F00) >> 8;
        let opcode_b2 = (opcode & 0x00F0) >> 4;
        let opcode_b3 = opcode & 0x000F;

        let vx = self.v[opcode_b1 as usize];
        let vy = self.v[opcode_b2 as usize];
        let n = (opcode_b3 & 0x000F) as u8;
        let kk = (opcode & 0x00FF) as u8;
        let nnn = opcode & 0x0FFF;

        match (opcode_b0, opcode_b1, opcode_b2, opcode_b3) {
            (0x0, 0x0, 0xE, 0x0) => {
                // 00E0 - CLS
                // Clear the display.
            }
            (0x0, 0x0, 0xE, 0xE) => {
                // 00EE - RET
                // Return from a subroutine.
                // The interpreter sets the program counter to the address at the top of the stack, then subtracts 1 from the stack pointer.
            }
            (0x0, _, _, _) => {
                // 0NNN - SYS addr
                // Jump to a machine code routine at nnn.
                // This instruction is only used on the old computers on which Chip-8 was originally implemented. It is ignored by modern interpreters.
                todo!();
            }
            (0x1, _, _, _) => {
                // 1NNN - JP addr
                // Jump to location nnn.
                // The interpreter sets the program counter to nnn.
            }
            (0x2, _, _, _) => {
                // 2NNN - CALL addr
                // Call subroutine at nnn.
                // The interpreter increments the stack pointer, then puts the current PC on the top of the stack. The PC is then set to nnn.
            }
            (0x3, _, _, _) => {
                // 3XKK - SE Vx, byte
                // Skip next instruction if Vx = kk.
                // The interpreter compares register Vx to kk, and if they are equal, increments the program counter by 2.
            }
            (0x4, _, _, _) => {
                // 4XKK - SNE Vx, byte
                // Skip next instruction if Vx != kk.
                // The interpreter compares register Vx to kk, and if they are not equal, increments the program counter by 2.
            }
            (0x5, _, _, 0x0) => {
                // 5XY0 - SE Vx, Vy
                // Skip next instruction if Vx = Vy.
                // The interpreter compares register Vx to register Vy, and if they are equal, increments the program counter by 2.
            }
            (0x6, _, _, _) => {
                // 6XKK - LD Vx, byte
                // Set Vx = kk.
                // The interpreter puts the value kk into register Vx.
            }
            (0x7, _, _, _) => {
                // 7XKK - ADD Vx, byte
                // Set Vx = Vx + kk.
                // Adds the value kk to the value of register Vx, then stores the result in Vx.
            }
            (0x8, _, _, 0x0) => {
                // 8XY0 - LD Vx, Vy
                // Set Vx = Vy.
                // Stores the value of register Vy in register Vx.
            }
            (0x8, _, _, 0x1) => {
                // 8XY1 - OR Vx, Vy
                // Set Vx = Vx OR Vy.
                // Performs a bitwise OR on the values of Vx and Vy, then stores the result in Vx. A bitwise OR compares the corrseponding bits from two values, and if either bit is 1, then the same bit in the result is also 1. Otherwise, it is 0.
            }
            (0x8, _, _, 0x2) => {
                // 8XY2 - AND Vx, Vy
                // Set Vx = Vx AND Vy.
                // Performs a bitwise AND on the values of Vx and Vy, then stores the result in Vx. A bitwise AND compares the corrseponding bits from two values, and if both bits are 1, then the same bit in the result is also 1. Otherwise, it is 0.
            }
            (0x8, _, _, 0x3) => {
                // 8XY3 - XOR Vx, Vy
                // Set Vx = Vx XOR Vy.
                // Performs a bitwise exclusive OR on the values of Vx and Vy, then stores the result in Vx. An exclusive OR compares the corrseponding bits from two values, and if the bits are not both the same, then the corresponding bit in the result is set to 1. Otherwise, it is 0.
            }
            (0x8, _, _, 0x4) => {
                // 8XY4 - ADD Vx, Vy
                // Set Vx = Vx + Vy, set VF = carry.
                // The values of Vx and Vy are added together. If the result is greater than 8 bits (i.e., > 255,) VF is set to 1, otherwise 0. Only the lowest 8 bits of the result are kept, and stored in Vx.
            }
            (0x8, _, _, 0x5) => {
                // 8XY5 - SUB Vx, Vy
                // Set Vx = Vx - Vy, set VF = NOT borrow.
                // If Vx > Vy, then VF is set to 1, otherwise 0. Then Vy is subtracted from Vx, and the results stored in Vx.
            }
            (0x8, _, _, 0x6) => {
                // 8XY6 - SHR Vx {, Vy}
                // Set Vx = Vx SHR 1.
                // If the least-significant bit of Vx is 1, then VF is set to 1, otherwise 0. Then Vx is divided by 2.
            }
            (0x8, _, _, 0x7) => {
                // 8XY7 - SUBN Vx, Vy
                // Set Vx = Vy - Vx, set VF = NOT borrow.
                // If Vy > Vx, then VF is set to 1, otherwise 0. Then Vx is subtracted from Vy, and the results stored in Vx.
            }
            (0x8, _, _, 0xE) => {
                // 8XYE - SHL Vx {, Vy}
                // Set Vx = Vx SHL 1.
                // If the most-significant bit of Vx is 1, then VF is set to 1, otherwise to 0. Then Vx is multiplied by 2.
            }
            (0x9, _, _, 0x0) => {
                // 9XY0 - SNE Vx, Vy
                // Skip next instruction if Vx != Vy.
                // The values of Vx and Vy are compared, and if they are not equal, the program counter is increased by 2.
            }
            (0xA, _, _, _) => {
                // ANNN - ld i, addr
                // set i = nnn.
                // the value of register i is set to nnn.
            }
            (0xB, _, _, _) => {
                // BNNN - JP V0, addr
                // Jump to location nnn + V0.
                // The program counter is set to nnn plus the value of V0.
            }
            (0xC, _, _, _) => {
                // CXKK - RND Vx, byte
                // Set Vx = random byte AND kk.
                // The interpreter generates a random number from 0 to 255, which is then ANDed with the value kk. The results are stored in Vx. See instruction 8xy2 for more information on AND.
            }
            (0xD, _, _, _) => {
                // Dxyn - DRW Vx, Vy, nibble
                // Display n-byte sprite starting at memory location I at (Vx, Vy), set VF = collision.
                // The interpreter reads n bytes from memory, starting at the address stored in I. These bytes are then displayed as sprites on screen at coordinates (Vx, Vy). Sprites are XORed onto the existing screen. If this causes any pixels to be erased, VF is set to 1, otherwise it is set to 0. If the sprite is positioned so part of it is outside the coordinates of the display, it wraps around to the opposite side of the screen. See instruction 8xy3 for more information on XOR, and section 2.4, Display, for more information on the Chip-8 screen and sprites.
            }
            (0xE, _, 0x9, 0xE) => {
                // Ex9E - SKP Vx
                // Skip next instruction if key with the value of Vx is pressed.
                // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the down position, PC is increased by 2.
            }
            (0xE, _, 0xA, 0x1) => {
                // ExA1 - SKNP Vx
                // Skip next instruction if key with the value of Vx is not pressed.
                // Checks the keyboard, and if the key corresponding to the value of Vx is currently in the up position, PC is increased by 2.
            }
            (0xF, _, 0x0, 0x7) => {
                // Fx07 - LD Vx, DT
                // Set Vx = delay timer value.
                // The value of DT is placed into Vx.
            }
            (0xF, _, 0x0, 0xA) => {
                // Fx0A - LD Vx, K
                // Wait for a key press, store the value of the key in Vx.
                // All execution stops until a key is pressed, then the value of that key is stored in Vx.
            }
            (0xF, _, 0x1, 0x5) => {
                // Fx15 - LD DT, Vx
                // Set delay timer = Vx.
                // DT is set equal to the value of Vx.
            }
            (0xF, _, 0x1, 0x8) => {
                // Fx18 - LD ST, Vx
                // Set sound timer = Vx.
                // ST is set equal to the value of Vx.
            }
            (0xF, _, 0x1, 0xE) => {
                // Fx1E - ADD I, Vx
                // Set I = I + Vx.
                // The values of I and Vx are added, and the results are stored in I.
            }
            (0xF, _, 0x2, 0x9) => {
                // Fx29 - LD F, Vx
                // Set I = location of sprite for digit Vx.
                // The value of I is set to the location for the hexadecimal sprite corresponding to the value of Vx. See section 2.4, Display, for more information on the Chip-8 hexadecimal font.
            }
            (0xF, _, 0x3, 0x3) => {
                // Fx33 - LD B, Vx
                // Store BCD representation of Vx in memory locations I, I+1, and I+2.
                // The interpreter takes the decimal value of Vx, and places the hundreds digit in memory at location in I, the tens digit at location I+1, and the ones digit at location I+2.
            }
            (0xF, _, 0x5, 0x5) => {
                // Fx55 - LD [I], Vx
                // Store registers V0 through Vx in memory starting at location I.
                // The interpreter copies the values of registers V0 through Vx into memory, starting at the address in I.
            }
            (0xF, _, 0x6, 0x5) => {
                // Fx65 - LD Vx, [I]
                // Read registers V0 through Vx from memory starting at location I.
                // The interpreter reads values from memory starting at location I into registers V0 through Vx.
            }
            (_, _, _, _) => {
                todo!()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
